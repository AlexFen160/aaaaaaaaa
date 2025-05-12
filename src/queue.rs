// grok-client/src/queue.rs
use std::{
    collections::VecDeque,
    sync::Arc,
};
use tokio::sync::Mutex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequestPriority {
    High = 2,
    Normal = 1,
    Low = 0,
}

pub struct BotRequest {
    pub message: grammers_client::InputMessage,
    pub priority: RequestPriority,
}

pub struct PriorityQueue {
    inner: Mutex<VecDeque<BotRequest>>,
}

impl PriorityQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(VecDeque::new()),
        })
    }

    pub async fn push(&self, request: BotRequest) {
        let mut queue = self.inner.lock().await;
        let pos = queue
            .iter()
            .position(|r| r.priority < request.priority)
            .unwrap_or(queue.len());
        queue.insert(pos, request);
    }

    pub async fn pop(&self) -> Option<BotRequest> {
        self.inner.lock().await.pop_front()
    }
}