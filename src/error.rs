use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrokError {
    #[error("Configuration error: {0}")]
    ConfigMissing(&'static str),

    #[error("Invalid configuration: {0}")]
    ConfigInvalid(&'static str),

    #[error("Telegram API error: {0}")]
    TelegramError(#[from] grammers_client::errors::ClientError),

    #[error("Authentication failed")]
    AuthError,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Timeout waiting for response")]
    ResponseTimeout,

    #[error("Bot not found")]
    BotNotFound,

    #[error("Invalid bot type")]
    InvalidBotType,
}