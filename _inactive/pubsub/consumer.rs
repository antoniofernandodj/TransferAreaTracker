use std::sync::MutexGuard;
use crate::pubsub::models::EventHandler;
use super::models::{PtEventHandler, PtEventStore};


pub struct Consumer {
    envent_store: PtEventStore,
    event_handler: PtEventHandler
}
impl Consumer {
    pub fn new(
        envent_store: PtEventStore,
        event_handler: PtEventHandler
    ) -> Self {
        Consumer{envent_store, event_handler}
    }

    pub fn run(&mut self) {
        println!("Running consumer...");
        loop {

            if let Ok(mut es) = self.envent_store.lock() {
                if let Some(event) = es.pop() {

                    println!("Consumed event {:?}", event);

                    let eh_guard: MutexGuard<EventHandler> = self.event_handler.lock().unwrap();

                    if let Some(
                        callback
                    ) = eh_guard.get(&event._type) {
                        callback(&event)
                    }
                }
            }
        }
    }
}
