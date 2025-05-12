use crate::{client::TelegramClient, error::GrokError, queue::BotRequest};
use grammers_client::{Client, types::PackedChat};
use tokio::time;

const RESPONSE_TIMEOUT: u64 = 30;

pub fn spawn_queue_processor(
    queue: Arc<super::queue::PriorityQueue>,
    client: Client,
    bot_username: String,
) {
    tokio::spawn(async move {
        let bot = match resolve_bot(&client, &bot_username).await {
            Ok(bot) => bot,
            Err(e) => {
                eprintln!("Failed to resolve bot: {}", e);
                return;
            }
        };

        loop {
            if let Some(request) = queue.pop().await {
                handle_message(&client, &bot, request).await;
            }
            time::sleep(time::Duration::from_millis(100)).await;
        }
    });
}

async fn resolve_bot(client: &Client, username: &str) -> Result<PackedChat, GrokError> {
    match client.resolve_username(username).await? {
        Some(grammers_client::types::Chat::User(user)) => Ok(user.into()),
        Some(_) => Err(GrokError::InvalidBotType),
        None => Err(GrokError::BotNotFound),
    }
}

async fn handle_message(client: &Client, bot: &PackedChat, request: BotRequest) {
    // Реализация обработки сообщений
}