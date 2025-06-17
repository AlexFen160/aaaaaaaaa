use thiserror::Error;
use grammers_client::{
    client::updates::AuthorizationError,
    SignInError,
    InvocationError
};

#[derive(Error, Debug)]
pub enum GrokError {
    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Session error: {0}")]
    Session(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Invocation error: {0}")]
    Invocation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Bot error: {0}")]
    Bot(String),
}

impl From<SignInError> for GrokError {
    fn from(e: SignInError) -> Self {
        GrokError::Auth(e.to_string())
    }
}

impl From<AuthorizationError> for GrokError {
    fn from(e: AuthorizationError) -> Self {
        GrokError::Authorization(e.to_string())
    }
}

impl From<InvocationError> for GrokError {
    fn from(e: InvocationError) -> Self {
        GrokError::Invocation(e.to_string())
    }
}