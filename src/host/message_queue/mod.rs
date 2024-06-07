mod message;

pub use message::project_viewed::ProjectViewed;
pub use message::create_tag::CreateTag;
pub use message::create_snapshot::CreateSnapshot;
pub use message::remove_tag::RemoveTag;
pub use message::assign_project::AssignProject;
pub use message::create_project::CreateProject;
use message::Queueable;

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
                    Ok(message) => if let Err(e) = message.handle().await {
                        error!("Error handling message: {}", e)
                    }
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

    pub async fn send(&self, message: impl Into<Message>) {
        _ = self.0
            .send(message.into())
            .await;
    }
}

impl Default for MessageQueueProducer {
    fn default() -> Self {
        MESSAGE_QUEUE
            .get_or_init(MessageQueue::new)
            .producer()
    }
}