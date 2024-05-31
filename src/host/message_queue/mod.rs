mod message;

pub use message::project_viewed::ProjectViewed;

use std::sync::OnceLock;

use async_channel::{Receiver, Sender};

use self::message::Message;

use log::error;
use tokio::{spawn, task::JoinHandle};

static MESSAGE_QUEUE: OnceLock<MessageQueue> = OnceLock::new();

pub struct MessageQueue {
    #[allow(dead_code)]
    consumer: MessageQueueConsumer,
    producer: MessageQueueProducer,
}

impl MessageQueue {
    pub fn new() -> Self {
        let (sender, reciever) = async_channel::unbounded();
        
        Self {
            consumer: MessageQueueConsumer::new(reciever),
            producer: MessageQueueProducer::new(sender),
        }
    }
    
    pub fn producer(&self) -> MessageQueueProducer {
        self.producer.clone()
    }
}

#[allow(dead_code)]
pub struct MessageQueueConsumer(JoinHandle<()>);

impl MessageQueueConsumer {
    pub fn new(reciever: Receiver<Message>) -> Self {
        Self(spawn(async move {
            loop {
                match reciever.recv().await {
                    Ok(message) => message.handle().await.unwrap_or_else(|e| error!("Error handling message: {}", e)),
                    Err(e) => error!("Error receiving message: {}", e),
                };
            }
        }))
    }
}

#[derive(Clone)]
pub struct MessageQueueProducer(Sender<Message>);

impl MessageQueueProducer {
    pub fn new(sender: Sender<Message>) -> Self {
        Self(sender)
    }

    pub async fn enqueue(&self, message: impl Into<Message>) {
        self.0.send(message.into()).await.expect("Error sending message");
    }
}

impl Default for MessageQueueProducer {
    fn default() -> Self {
        let queue = MESSAGE_QUEUE.get_or_init(MessageQueue::new);

        queue.producer()
    }
}