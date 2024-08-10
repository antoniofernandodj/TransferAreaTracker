// mod keyboard;
// mod template;

use std::{collections::HashMap, sync::{Arc, Mutex}};

pub type EventStore = Vec<Event>;
pub type PtEventStore = Arc<Mutex<EventStore>>;

pub type EventHandler = HashMap<EventType, Box<dyn Fn(&Event) + Send>>;
pub type PtEventHandler = Arc<Mutex<EventHandler>>;


#[derive(Eq, Hash, PartialEq, Debug)]
pub enum EventType {
    VeryLargeNumber,
    VerySmallNumber
}


#[derive(Debug)]
pub struct Event {
    pub(crate) _type: EventType,
    pub(crate) _data: HashMap<String, f64>
}
