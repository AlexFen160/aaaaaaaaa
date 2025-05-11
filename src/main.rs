use dotenv::dotenv;
use grammers_client::{
    types::{Chat, PackedChat},
    Client, Config, InputMessage, Update,
};
use grammers_session::Session;
use std::collections::VecDeque;
use std::env;
use std::io::{self, Write};
use std::process;
use std::sync::Arc;
use tokio::sync::Mutex;

const SESSION_FILE: &str = "session.session";
const BOT_USERNAME: &str = "GrokAI";
const RESPONSE_TIMEOUT: u64 = 30;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RequestPriority {
    High = 2,
    Normal = 1,
    Low = 0,
}

struct BotRequest {
    message: InputMessage,
    priority: RequestPriority,
}

struct PriorityQueue {
    queue: Mutex<VecDeque<BotRequest>>,
}

impl PriorityQueue {
    fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    async fn push(&self, request: BotRequest) {
        let mut queue = self.queue.lock().await;
        let pos = queue
            .iter()
            .position(|r| r.priority < request.priority)
            .unwrap_or(queue.len());
        queue.insert(pos, request);
    }

    async fn pop(&self) -> Option<BotRequest> {
        self.queue.lock().await.pop_front()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let (api_id, api_hash) = load_credentials()?;
    println!("🚀 Starting with API_ID: {} and API_HASH: {}", api_id, api_hash);

    let client = init_client(api_id, &api_hash).await?;
    let bot = resolve_bot(&client).await?;

    let queue = Arc::new(PriorityQueue::new());
    spawn_queue_processor(queue.clone(), client.clone(), bot.clone());
    load_test_messages(queue.clone(), client.clone()).await?;

    tokio::signal::ctrl_c().await?;
    println!("🛑 Program stopped");
    Ok(())
}

fn load_credentials() -> Result<(i32, String), Box<dyn std::error::Error>> {
    let api_id = env::var("API_ID")
        .or_else(|_| dotenv::var("API_ID"))
        .map_err(|_| Box::<dyn std::error::Error>::from("API_ID not set!"))?
        .parse()
        .map_err(|_| Box::<dyn std::error::Error>::from("Invalid API_ID format"))?;

    let api_hash = env::var("API_HASH")
        .or_else(|_| dotenv::var("API_HASH"))
        .map_err(|_| Box::<dyn std::error::Error>::from("API_HASH not set!"))?;

    Ok((api_id, api_hash))
}

async fn init_client(
    api_id: i32,
    api_hash: &str,
) -> Result<Client, Box<dyn std::error::Error>> {
    let session = Session::load_file_or_create(SESSION_FILE)?;
    let client = Client::connect(Config {
        session,
        api_id,
        api_hash: api_hash.to_string(),
        params: Default::default(),
    })
        .await?;

    if !client.is_authorized().await? {
        authorize_client(&client).await?;
    }

    Ok(client)
}

async fn authorize_client(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    use grammers_client::SignInError;

    println!("🔑 Authorization required");
    let phone = prompt("Enter phone number: ")?;
    let token = client.request_login_code(&phone).await?;
    let code = prompt("Enter Telegram code: ")?;

    match client.sign_in(&token, &code).await {
        Ok(_) => {
            println!("🔒 Authorization successful!");
        }
        Err(SignInError::PasswordRequired(password_token)) => {
            println!("🔐 Two-factor authentication enabled");
            let password = prompt("Enter your 2FA password: ")?;
            client.check_password(password_token, &password).await?;
            println!("🔓 2FA authentication successful!");
        }
        Err(e) => return Err(Box::new(e)),
    }

    client
        .session()
        .save_to_file(SESSION_FILE)
        .map_err(|e| format!("Failed to save session: {}", e))?;

    Ok(())
}


async fn resolve_bot(client: &Client) -> Result<PackedChat, Box<dyn std::error::Error>> {
    println!("🔍 Searching for bot {}...", BOT_USERNAME);
    match client.resolve_username(BOT_USERNAME).await? {
        Some(Chat::User(user)) => {
            println!("✅ Bot found: {} (ID: {})", user.first_name(), user.id());
            Ok(user.into())
        }
        Some(_) => {
            eprintln!("❌ {} is not a user bot!", BOT_USERNAME);
            process::exit(1);
        }
        None => {
            eprintln!("❌ Bot {} not found!", BOT_USERNAME);
            process::exit(1);
        }
    }
}

async fn load_test_messages(
    queue: Arc<PriorityQueue>,
    client: Client,
) -> Result<(), Box<dyn std::error::Error>> {
    queue
        .push(BotRequest {
            message: InputMessage::text("🚨 High priority message!"),
            priority: RequestPriority::High,
        })
        .await;

    queue
        .push(BotRequest {
            message: InputMessage::text("📨 Normal message"),
            priority: RequestPriority::Normal,
        })
        .await;

    match client.upload_file("image.jpg").await {
        Ok(media) => {
            queue
                .push(BotRequest {
                    message: InputMessage::text("📸 Photo with description").document(media),
                    priority: RequestPriority::Low,
                })
                .await;
        }
        Err(e) => eprintln!("⚠️ Failed to upload media: {}", e),
    }

    Ok(())
}

fn spawn_queue_processor(queue: Arc<PriorityQueue>, client: Client, bot: PackedChat) {
    tokio::spawn(async move {
        println!("⏳ Queue processor started");
        loop {
            if let Some(request) = queue.pop().await {
                handle_message(&client, &bot, request).await;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });
}

async fn handle_message(client: &Client, bot: &PackedChat, request: BotRequest) {
    println!("📤 Sending message with priority {:?}", request.priority);

    match client.send_message(bot.clone(), request.message).await {
        Ok(_) => {
            println!("✅ Message sent");
            // ✅ Исправлено здесь:
            if let Err(e) = wait_for_response(client, bot.id).await {
                eprintln!("❌ Response error: {}", e);
            }
        }
        Err(e) => eprintln!("❌ Send error: {}", e),
    }
}

async fn wait_for_response(client: &Client, bot_id: i64) -> Result<(), Box<dyn std::error::Error>> {
    println!("⏳ Waiting for response...");
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < std::time::Duration::from_secs(RESPONSE_TIMEOUT) {
        if let Update::NewMessage(message) = client.next_update().await? {
            if let Some(sender) = message.sender() {
                if sender.id() == bot_id {
                    println!("📥 Bot response: {}", message.text());
                    return Ok(());
                }
            }
        }
    }

    Err("⌛ Response timeout".into())
}

fn prompt(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", text);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
