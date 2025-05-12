use crate::{config::Config, error::GrokError, queue::PriorityQueue};
use grammers_client::{Client, Config as TgConfig, SignInError};
use grammers_session::Session;
use std::sync::Arc;

pub struct TelegramClient {
    inner: Client,
    queue: Arc<PriorityQueue>,
    config: Config,
}

impl TelegramClient {
    pub async fn new(config: Config) -> Result<Self, GrokError> {
        let session = Session::load_file_or_create(&config.session_file)?;
        let client = Client::connect(TgConfig {
            session,
            api_id: config.api_id,
            api_hash: config.api_hash.clone(),
            params: Default::default(),
        })
            .await?;

        if !client.is_authorized().await? {
            Self::authorize(&client).await?;
        }

        Ok(Self {
            inner: client,
            queue: Arc::new(PriorityQueue::new()),
            config,
        })
    }

    async fn authorize(client: &Client) -> Result<(), GrokError> {
        // Реализация авторизации аналогична предыдущей версии
        // с обработкой ошибок через GrokError
    }

    pub async fn start_processing(&self) {
        crate::handlers::spawn_queue_processor(
            self.queue.clone(),
            self.inner.clone(),
            self.config.bot_username.clone(),
        );
    }

    pub async fn send_message(&self, text: &str, priority: crate::queue::RequestPriority) {
        self.queue.push(BotRequest {
            message: grammers_client::InputMessage::text(text),
            priority,
        }).await;
    }
}