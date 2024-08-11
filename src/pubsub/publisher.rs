// mod keyboard;
// mod template;

use crate::pubsub::models::Event;

use std::collections::{HashMap, HashSet};
use std::process::Command;
use clipboard::ClipboardProvider;
use device_query::{DeviceQuery, Keycode};
use super::models::{EventType, PtEventStore};
use device_query;


pub struct Publisher {
    event_store: PtEventStore,

}

impl Publisher {
    pub fn new(
        event_store: PtEventStore,

    ) -> Self {
        Publisher{event_store}
    }

    pub fn run(&mut self) {
        clear_screen();

        let mut last_active_history_index: usize = 10;
        let mut active_history_index: usize = 1;
    
        let device_state: device_query::DeviceState = device_query::DeviceState::new();
        let mut ctx: clipboard::ClipboardContext = clipboard::ClipboardProvider::new().unwrap();
        let mut last_clipboard_content: String = ctx.get_contents().unwrap_or_default();
        let previous_keys: HashSet<Keycode> = HashSet::new();
        let mut pk = previous_keys;
        
        println!("Listening publisher...");
        loop {

            let current_keys: HashSet<Keycode> = device_state
                .get_keys()
                .into_iter()
                .collect();

            // Alias
            let mut ck = current_keys;

            /* UpdatedPreviousKeysSet */ {

                if updated_previous_keys_set(
                    &mut ck,
                    &mut pk
                ) {

                    /* HistorySelected */ {

                        if let Some(new_history_index) = control_shift_number_pressed(
                            &ck
                        ) {
            
                            active_history_index = new_history_index as usize;
            
                            let event: Event = Event {
                                event_type: EventType::HistorySelected,
                                history: Some(active_history_index),
                                last_clipboard_content: None
                            };
            
                            if let Ok(mut es) = self.event_store.lock() {
                                es.push(event);
                            }
                
                        }

                    }
    
                }

            }

            /* History Size Chaged */ {

                if let Ok(current_clipboard_content) = ctx.get_contents() {
    
                    if clipboard_changed(
                        &mut last_clipboard_content,
                        &current_clipboard_content
                    ) {
    
                        let event: Event = Event {
                            event_type: EventType::NewItemInHistory,
                            history: None,
                            last_clipboard_content: Some(current_clipboard_content.clone())
                        };
        
                        if let Ok(mut es) = self.event_store.lock() {
                            es.push(event);
                        }

                        last_clipboard_content = current_clipboard_content;
    
                    }
                }

            }

            if last_active_history_index != active_history_index {
                last_active_history_index = active_history_index
            }

        }
    }
}

fn clear_screen() {
    let mut command: Command = Command::new("clear");
    let mut result: std::process::Child = command.spawn().unwrap();
    result.wait().unwrap();
}

pub fn control_shift_number_pressed(
    keys: &HashSet<Keycode>
) -> Option<i32> {

    let number_mapping: HashMap<Keycode, i32> = HashMap::from([
        (Keycode::Key1, 1),
        (Keycode::Key2, 2),
        (Keycode::Key3, 3),
        (Keycode::Key4, 4),
        (Keycode::Key5, 5),
        (Keycode::Key6, 6),
        (Keycode::Key7, 7),
        (Keycode::Key8, 8),
        (Keycode::Key9, 9),
        (Keycode::Key0, 0)
    ]);
        
    let control_pressed: bool = keys.contains(
        &Keycode::LControl) || keys.contains(&Keycode::RControl
    );

    let shift_pressed: bool = keys.contains(
        &Keycode::LShift) || keys.contains(&Keycode::RShift
    );

    let shift_and_control_pressed: bool = shift_pressed && control_pressed;

    // println!("shift_and_control_pressed: {shift_and_control_pressed}");

    for key in keys {
        match key {
            Keycode::Key1 | Keycode::Key2 |
            Keycode::Key3 | Keycode::Key4 |
            Keycode::Key5 | Keycode::Key6 |
            Keycode::Key7 | Keycode::Key8 |
            Keycode::Key9 | Keycode::Key0 => {

                if shift_and_control_pressed {
                    // println!("Pressed Shift + Control + {:?}", key);

                    match number_mapping.get(key) {
                        Some(v) => {
                            // println!("Some {v}");
                            return Some(v.clone())
                        },
                        None => {
                            // println!("None");
                            return None
                        }
                        
                    };

                }

            }
            _ => { }
        }
    }
    return None
}

pub fn clipboard_changed(
    last_clipboard_content: &mut String,
    current_clipboard_content: &String
) -> bool {

    if (&current_clipboard_content != &last_clipboard_content) &&
        *current_clipboard_content != "".to_string() {
            return true
    }

    false
}

pub fn updated_previous_keys_set(
    current_keys: &mut HashSet<Keycode>,
    previous_keys: &mut HashSet<Keycode>
) -> bool {
    if previous_keys != current_keys {
        *previous_keys = current_keys.clone();
        true
    } else {
        false
    }
}
