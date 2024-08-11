// mod keyboard;
// mod template;

use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::HistoryGroup;

pub type EventStore = Vec<Event>;
pub type PtEventStore = Arc<Mutex<EventStore>>;

pub type EventHandler = HashMap<EventType, Box<dyn Fn(&Event, &PtHistoryGroup) + Send>>;
pub type PtEventHandler = Arc<Mutex<EventHandler>>;
pub type PtHistoryGroup = Arc<Mutex<HistoryGroup>>;


#[derive(Eq, Hash, PartialEq, Debug)]
pub enum EventType {
    NewItemInHistory,
    HistorySelected
}


#[derive(Debug)]
pub struct Event {
    pub(crate) event_type: EventType,
    pub(crate) history: Option<usize>,
    pub(crate) last_clipboard_content: Option<String>
}
