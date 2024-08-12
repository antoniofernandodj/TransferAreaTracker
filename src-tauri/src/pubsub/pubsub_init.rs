use std::{collections::HashMap, sync::{Arc, Mutex}};

use super::{pubsub_consumer::Consumer, pubsub_controllers, pubsub_models::{EventHandler, EventType, PtEventHandler, PtEventStore, PtHistoryGroup}, pubsub_publisher::Publisher};


pub fn init_pubsub(history_group: PtHistoryGroup) -> (PtHistoryGroup, Publisher, Consumer) {

    let event_store: PtEventStore = Arc::new(Mutex::new(vec![]));
    let mut eh: EventHandler = HashMap::from([]);

    /* Cadastrar event handlers */ {
        eh.insert(
            EventType::NewItemInHistory,
            Box::new(pubsub_controllers::new_item_in_history)
        );
    
        eh.insert(
            EventType::HistorySelected,
            Box::new(pubsub_controllers::history_selected)
        );
    }

    let event_handler: PtEventHandler = Arc::new(Mutex::new(eh));

    let es1: PtEventStore = Arc::clone(&event_store);
    let es2: PtEventStore = Arc::clone(&event_store);
    let eh: PtEventHandler = Arc::clone(&event_handler);
    let hg: PtHistoryGroup = Arc::clone(&history_group);

    let publisher: Publisher = Publisher::new(es1);
    let consumer: Consumer = Consumer::new(
        hg,
        eh,
        es2
    );

    (history_group, publisher, consumer)

}
