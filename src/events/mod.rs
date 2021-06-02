use std::collections::HashMap;

use crossbeam::channel::{Receiver, Sender};
use eyre::Result;

use self::event::Event;

pub mod event;

pub struct EventManager {
    event_receiver: Receiver<Event>,
    event_sender: Sender<Event>,
    event_subscribers: HashMap<String, Vec<Sender<Event>>>,
}

impl EventManager {
    pub fn new() -> Self {
        let (event_sender, event_receiver) = crossbeam::channel::unbounded();
        let event_subscribers = HashMap::new();

        Self {
            event_receiver,
            event_sender,
            event_subscribers,
        }
    }

    pub fn register(&self) -> Sender<Event> {
        self.event_sender.clone()
    }

    pub fn process(&mut self) -> Result<()> {
        while let Ok(event) = self.event_receiver.try_recv() {
            if let Some(subscribers) = self.event_subscribers.get_mut(event.as_ref()) {
                subscribers
                    .iter_mut()
                    .try_for_each(|subscriber| subscriber.send(event.clone()))?;
            }
        }
        Ok(())
    }

    pub fn subscribe(&mut self, events_to_subscribe_to: Vec<String>) -> Receiver<Event> {
        let (sender, receiver) = crossbeam::channel::unbounded();
        events_to_subscribe_to
            .into_iter()
            .for_each(|stringified_event| {
                let subscribers = self.event_subscribers.entry(stringified_event).or_default();
                subscribers.push(sender.clone());
            });

        receiver
    }
}
