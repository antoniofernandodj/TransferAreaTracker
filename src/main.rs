mod pubsub;

use std::{collections::HashMap, sync::{Arc, Mutex}, thread};

// use pubsub::consumer::Consumer;
use pubsub::{consumer::Consumer, controllers, models::{EventHandler, PtEventHandler, PtEventStore, PtHistoryGroup}, publisher::Publisher};
// use rand::rngs::ThreadRng;

#[derive(Debug)]
struct HistoryGroup {
    hs0: Vec::<String>,
    hs1: Vec::<String>,
    hs2: Vec::<String>,
    hs3: Vec::<String>,
    hs4: Vec::<String>,
    hs5: Vec::<String>,
    hs6: Vec::<String>,
    hs7: Vec::<String>,
    hs8: Vec::<String>,
    hs9: Vec::<String>,
    active: i32
}

impl HistoryGroup {
    fn new() -> Self {
        HistoryGroup {
            hs0: Vec::<String>::new(),
            hs1: Vec::<String>::new(),
            hs2: Vec::<String>::new(),
            hs3: Vec::<String>::new(),
            hs4: Vec::<String>::new(),
            hs5: Vec::<String>::new(),
            hs6: Vec::<String>::new(),
            hs7: Vec::<String>::new(),
            hs8: Vec::<String>::new(),
            hs9: Vec::<String>::new(),
            active: 1
        }
    }

    fn get_active_history_number(&self) -> i32 {
        self.active
    }

    fn get_active_history(&mut self) -> &mut Vec<String> {
        if self.active == 1 { return &mut self.hs1 }
        else if self.active == 2 { return &mut self.hs2 }
        else if self.active == 3 { return &mut self.hs3 }
        else if self.active == 4 { return &mut self.hs4 }
        else if self.active == 5 { return &mut self.hs5 }
        else if self.active == 6 { return &mut self.hs6 }
        else if self.active == 7 { return &mut self.hs7 }
        else if self.active == 8 { return &mut self.hs8 }
        else if self.active == 9 { return &mut self.hs9 }
        else { return &mut self.hs0 }
    }

    fn set_active(&mut self, n: i32) {
        self.active = n
    }

}

fn main() {
    let mut eh: EventHandler = HashMap::from([]);

    /* Cadastrar event handlers */ {
        eh.insert(
            pubsub::models::EventType::NewItemInHistory,
            Box::new(controllers::history_new_item_in_history)
        );
    
        eh.insert(
            pubsub::models::EventType::HistorySelected,
            Box::new(controllers::history_selected)
        );
    }

    let event_handler: PtEventHandler = Arc::new(Mutex::new(eh));
    let event_store: PtEventStore = Arc::new(Mutex::new(vec![]));
    let history_group = Arc::new(
        Mutex::new(HistoryGroup::new())
    );

    let es1: PtEventStore = Arc::clone(&event_store);
    let es2: PtEventStore = Arc::clone(&event_store);
    let eh: PtEventHandler = Arc::clone(&event_handler);
    let hg: PtHistoryGroup = Arc::clone(&history_group);


    let mut p: Publisher = Publisher::new(es1);
    let mut c: Consumer = Consumer::new(hg, eh, es2);

    let publisher_handle: thread::JoinHandle<()> = thread::spawn(
        move || { p.run() }
    );

    let consumer_handler: thread::JoinHandle<()> = thread::spawn(
        move || { c.run(); }
    );

    let _ = publisher_handle.join();
    let _ = consumer_handler.join();

}