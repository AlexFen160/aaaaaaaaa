use crate::client::GrokClient;

pub trait MessageHandler: Send + Sync {
    fn handle(&self, message: &str);
}

impl GrokClient {
    pub fn add_custom_handler<H: MessageHandler>(&self, handler: H) {
        // Регистрация кастомного обработчика
    }
}