// mod keyboard;
// mod template;

use rand::Rng;
use std::collections::HashMap;
use super::models::{EventType, PtEventStore};
use super::models::Event;


pub struct Publisher {
    envent_store: PtEventStore
}
impl Publisher {
    pub fn new(envent_store: PtEventStore) -> Self {
        Publisher{envent_store}
    }

    pub fn run(&mut self) {

        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        println!("Running publisher...");
        loop {
            
            let value: f64 = rng.gen::<f64>();
            let threshold_plus: f64 = 0.99999995;
            let threshold_minus: f64 = 0.00000007;

            if value > threshold_plus {

                let event: Event = Event {
                    _type: EventType::VeryLargeNumber,
                    _data: HashMap::from([
                        (String::from("value"), value)
                    ])
                };

                if let Ok(mut es) = self.envent_store.lock() {
                    // println!("Publishing event {:?}...", event);
                    es.push(event);

                }

            }

            if value < threshold_minus {

                let event: Event = Event {
                    _type: EventType::VerySmallNumber,
                    _data: HashMap::from([
                        (String::from("value"), value)
                    ])
                };

                if let Ok(mut es) = self.envent_store.lock() {
                    // println!("Publishing event {:?}...", event);
                    es.push(event);
                }

            }

        }
    }
}

