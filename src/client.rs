use std::io;
use grammers_client::{
    Client, Config, InputMessage, Update,
    types::{Chat, PackedChat},
};
use grammers_session::Session;
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::{
    config::GrokConfig,
    error::GrokError,
    queue::{PriorityQueue, RequestPriority},
};

pub struct GrokClient {
    client: Client,
    queue: Arc<Mutex<PriorityQueue>>,
    bot: PackedChat,
    bot_id: i64,
}

impl GrokClient {
    pub async fn new(config: GrokConfig) -> Result<Self, GrokError> {
        let session = Session::load_file_or_create(&config.session_path)
            .map_err(|e| GrokError::Session(e.to_string()))?;

        let client = Client::connect(Config {
            session,
            api_id: config.api_id,
            api_hash: config.api_hash.clone(),
            params: Default::default(),
        })
            .await
            .map_err(|e| GrokError::Connection(e.to_string()))?;

        if !client.is_authorized().await? {
            Self::authorize(&client).await?;
        }

        let (bot, bot_id) = Self::resolve_bot(&client, &config.bot_username).await?;

        Ok(Self {
            client: client.clone(),
            queue: Arc::new(Mutex::new(PriorityQueue::new())),
            bot,
            bot_id,
        })
    }

    pub async fn send(&self, text: &str, priority: RequestPriority) -> Result<(), GrokError> {
        let message = InputMessage::text(text);
        let mut queue = self.queue.lock().await;
        queue.push(message, priority);
        Ok(())
    }

    pub fn start(&self) {
        let client = self.client.clone();
        let queue = self.queue.clone();
        let bot = self.bot.clone();
        let bot_id = self.bot_id;

        // Message sender
        tokio::spawn(async move {
            loop {
                let msg = {
                    let mut queue = queue.lock().await;
                    queue.pop()
                };

                if let Some((message, priority)) = msg {
                    match client.send_message(bot.clone(), message).await {
                        Ok(_) => log::info!("Sent (priority: {:?})", priority),
                        Err(e) => log::error!("Send error: {}", e),
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });

        // Message listener
        let client = self.client.clone();
        tokio::spawn(async move {
            loop {
                match client.next_update().await {
                    Ok(Update::NewMessage(message)) => {
                        if let Some(sender) = message.sender() {
                            if sender.id() == bot_id {
                                println!("\n[Bot]: {}", message.text());
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(e) => log::error!("Update error: {}", e),
                }
            }
        });
    }

    async fn resolve_bot(client: &Client, username: &str) -> Result<(PackedChat, i64), GrokError> {
        let chat = client
            .resolve_username(username)
            .await?
            .ok_or_else(|| GrokError::Authorization(format!("Bot {} not found", username)))?;

        let bot_id = match &chat {
            Chat::User(user) => user.id(),
            _ => return Err(GrokError::Bot("Target is not a user bot".into())),
        };

        Ok((chat.pack(), bot_id))
    }

    async fn authorize(client: &Client) -> Result<(), GrokError> {
        use grammers_client::SignInError;

        println!("Enter phone number (e.g. +1234567890):");
        let phone = read_input()?;

        let token = client.request_login_code(&phone).await?;

        println!("Enter Telegram code:");
        let code = read_input()?;

        match client.sign_in(&token, &code).await {
            Ok(_) => {
                client.session().save_to_file("session.session")?;
                println!("Authorization successful!");
                Ok(())
            }
            Err(SignInError::PasswordRequired(token)) => {
                println!("Enter 2FA password:");
                let password = read_input()?;
                client.check_password(token, &password).await?;
                client.session().save_to_file("session.session")?;
                println!("2FA authentication successful!");
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}

fn read_input() -> Result<String, GrokError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}