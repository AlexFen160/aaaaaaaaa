use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct GrokConfig {
    pub api_id: i32,
    pub api_hash: String,
    pub bot_username: String,
    pub session_path: PathBuf,
    pub response_timeout: u64,
}

impl GrokConfig {
    pub fn new(
        api_id: i32,
        api_hash: impl Into<String>,
        bot_username: impl Into<String>,
        session_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            api_id,
            api_hash: api_hash.into(),
            bot_username: bot_username.into(),
            session_path: session_path.into(),
            response_timeout: 30,
        }
    }
}