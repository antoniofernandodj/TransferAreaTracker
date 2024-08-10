pub mod models;
pub mod consumer;
pub mod controllers;
pub mod publisher;

// mod keyboard;
// mod template;

use std::{collections::HashMap, sync::{Arc, Mutex}};
use std::thread;
use crate::pubsub::{
    consumer::Consumer,
    publisher::Publisher,
    models::{
        EventHandler,
        EventType,
        PtEventHandler,
        PtEventStore
    }
};


pub fn main() {

    let mut eh: EventHandler = HashMap::from([]);

    eh.insert(
        EventType::VeryLargeNumber,
        Box::new(controllers::numero_grande_callback)
    );

    eh.insert(
        EventType::VerySmallNumber,
        Box::new(controllers::numero_pequeno_callback)
    );

    let event_handler: PtEventHandler = Arc::new(Mutex::new(eh));
    let event_store: PtEventStore = Arc::new(Mutex::new(vec![]));

    let es1: PtEventStore = Arc::clone(&event_store);
    let es3: PtEventStore = Arc::clone(&event_store);
    let eh: PtEventHandler = Arc::clone(&event_handler);

    let mut consumer1: Consumer = Consumer::new(es1, eh);
    let mut publisher: Publisher = Publisher::new(es3);

    let consumer1_handle: thread::JoinHandle<()> = thread::spawn(move || {
        consumer1.run();
    });


    let publisher_handle: thread::JoinHandle<()> = thread::spawn(move || {
        publisher.run();
    });

    consumer1_handle.join().expect("Cannot run");
    publisher_handle.join().expect("Cannot run");

}