use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_id: i32,
    pub api_hash: String,
    pub bot_username: String,
    pub session_file: String,
}

impl Config {
    pub fn from_env() -> Result<Self, crate::error::GrokError> {
        dotenv().ok();

        Ok(Self {
            api_id: env::var("API_ID")
                .map_err(|_| crate::error::GrokError::ConfigMissing("API_ID"))?
                .parse()
                .map_err(|_| crate::error::GrokError::ConfigInvalid("API_ID"))?,

            api_hash: env::var("API_HASH")
                .map_err(|_| crate::error::GrokError::ConfigMissing("API_HASH"))?,

            bot_username: env::var("BOT_USERNAME")
                .unwrap_or_else(|_| "GrokAI".to_string()),

            session_file: env::var("SESSION_FILE")
                .unwrap_or_else(|_| "session.session".to_string()),
        })
    }
}