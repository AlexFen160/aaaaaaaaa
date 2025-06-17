use grok_client::prelude::*;

#[tokio::main]
async fn main() -> Result<(), GrokError> {
    env_logger::init();

    let config = GrokConfig::new(
        ************,          // Ваш API_ID
        "*********************", // Ваш API_HASH
        "GrokAI",      // Имя бота без @
        "session.session"
    );

    let client = GrokClient::new(config).await?;
    client.start();

    client.send("Hello!", RequestPriority::High).await?; // "Hello!" зпменить на текстовый вход

    tokio::signal::ctrl_c().await?;
    Ok(())
}