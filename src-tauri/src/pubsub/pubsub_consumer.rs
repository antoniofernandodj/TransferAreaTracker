use std::sync::MutexGuard;
use crate::pubsub::pubsub_models::EventHandler;
use super::pubsub_models::{PtEventHandler, PtEventStore, PtHistoryGroup};


pub struct Consumer {
    history_group: PtHistoryGroup,
    event_handler: PtEventHandler,
    event_store: PtEventStore,
}
impl Consumer {
    pub fn new(
        history_group: PtHistoryGroup,
        event_handler: PtEventHandler,
        event_store: PtEventStore
    ) -> Self {
        Consumer{history_group, event_handler, event_store}
    }

    pub fn run(&mut self) {
        println!("Running consumer...");
        loop {

            if let Ok(mut es) = self.event_store.lock() {
                if let Some(event) = es.pop() {

                    // println!("Evento consumido: {:?}\n\n", event);

                    let eh: MutexGuard<EventHandler> = self.event_handler.lock().unwrap();

                    if let Some(
                        cb
                    ) = eh.get(&event.event_type) {
                        cb(&event, &self.history_group)
                    }
                }
            }
        }
    }
}
