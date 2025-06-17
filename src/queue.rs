use grammers_client::InputMessage;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequestPriority {
    Emergency = 5,
    High = 3,
    Normal = 2,
    Low = 1,
}

// Убрать #[derive(Debug)]
struct QueueItem {
    message: InputMessage,
    priority: RequestPriority,
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for QueueItem {}

pub struct PriorityQueue {
    inner: BinaryHeap<QueueItem>,
}

impl PriorityQueue {
    pub fn new() -> Self {
        Self {
            inner: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, message: InputMessage, priority: RequestPriority) {
        self.inner.push(QueueItem { message, priority });
    }

    pub fn pop(&mut self) -> Option<(InputMessage, RequestPriority)> {
        self.inner.pop().map(|item| (item.message, item.priority))
    }
}