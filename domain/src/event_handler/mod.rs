use tokio::sync::mpsc;

use external_contracts::{
    event_handler::{Event, EventBus, EventError, EventListener},
    preinstalled::async_trait::async_trait,
};

pub struct EventBusImpl {
    sender: mpsc::Sender<Event>,
}

impl EventBusImpl {
    pub fn new() -> (Self, mpsc::Receiver<Event>) {
        let (sender, receiver) = mpsc::channel(100);
        (Self { sender }, receiver)
    }
}

#[async_trait]
impl EventBus for EventBusImpl {
    async fn publish(&self, event: Event) -> Result<(), EventError> {
        self.sender
            .send(event)
            .await
            .map_err(|_| EventError::ErrorPublishingEvent)
    }
}

pub struct EventListenerImpl {
    receiver: mpsc::Receiver<Event>,
}

impl EventListenerImpl {
    pub fn new(receiver: mpsc::Receiver<Event>) -> Self {
        Self { receiver }
    }
}

#[async_trait]
impl EventListener for EventListenerImpl {
    async fn listen(&mut self) {
        while let Some(event) = self.receiver.recv().await {
            println!("Received event: {:?}", event);
        }
    }
}
