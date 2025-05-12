
use grok_client::{Config, TelegramClient, RequestPriority};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    let client = TelegramClient::new(config).await?;
    client.start_processing().await;

    client.send_message("High priority message!", RequestPriority::High).await;
    client.send_message("Normal message", RequestPriority::Normal).await;

    tokio::signal::ctrl_c().await?;
    Ok(())
}