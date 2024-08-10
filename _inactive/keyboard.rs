use std::collections::HashSet;
use std::process::Command;
use std::collections::HashMap;
use clipboard::ClipboardProvider;
use device_query::{DeviceQuery, Keycode};
use device_query;



#[allow(dead_code)]
#[allow(unused_assignments)]
pub fn main() {
    clear_screen();

    let mut last_active_history_index: usize = 10;
    let mut active_history_index: usize = 1;

    let mut history_array: [Vec<String>; 10] = [
        Vec::<String>::new(),
        Vec::<String>::new(),
        Vec::<String>::new(),
        Vec::<String>::new(),
        Vec::<String>::new(),
        Vec::<String>::new(),
        Vec::<String>::new(),
        Vec::<String>::new(),
        Vec::<String>::new(),
        Vec::<String>::new(),
    ];

    let mut history_ref: &mut Vec<String> = &mut history_array[active_history_index];
    let device_state: device_query::DeviceState = device_query::DeviceState::new();
    let mut ctx: clipboard::ClipboardContext = clipboard::ClipboardProvider::new().unwrap();
    let mut last_clipboard_content: String = ctx.get_contents().unwrap_or_default();
    let mut previous_keys: HashSet<Keycode> = HashSet::new();
    let mut history_item_selected: bool = false;
    let mut last_item_index = 0;
    
    println!("Listening...");

    loop {
        history_item_selected = false;
        let mut current_keys: HashSet<Keycode> = device_state.get_keys().into_iter().collect();

        if let Some(_keyup) = keyup_event(
            &current_keys,
            &previous_keys
        ) {
            // println!("keyup: {_keyup}");
        }

        if let Some(_keydown) = keydown_event(
            &current_keys,
            &previous_keys
        ) {
            // println!("keydown: {_keydown}");
        }

        if updated_previous_keys_set(
            &mut current_keys, &mut previous_keys
        ) {
            // println!("current_keys: {current_keys:?}")
        }

        if let Some(new_history_index) = control_shift_number_pressed(
            &current_keys
        ) {
            active_history_index = new_history_index.clone() as usize;
            history_ref = &mut history_array[active_history_index];
            clear_screen();
            println!("Selected history[{active_history_index}]: {history_ref:#?}");

            let last_item: Option<&String> = history_ref.get(history_ref.len());
            if let Some(item) = last_item {
                
                ctx.set_contents(item.to_string()).unwrap();
                println!("Ajustando \"{item}\" para a area de transferência.");
                last_item_index = new_history_index;
            }
        }

        if let Some(new_item_index) = control_alt_number_pressed(&current_keys) {

            history_ref = &mut history_array[active_history_index];
            let index: usize = history_ref.len() - new_item_index as usize;

            if let Some(selected_content) = history_ref.get(index) {

                if last_item_index != new_item_index {
                    ctx.set_contents(selected_content.to_owned()).unwrap();
                    println!("Ajustando \"{selected_content}\" para a area de transferência.");
                    last_item_index = new_item_index;
                    history_item_selected = true;

                    if let Ok(current_content) = ctx.get_contents() {
                        last_clipboard_content = current_content;
                    }
                }
            }
        }

        if last_active_history_index != active_history_index {
            last_active_history_index = active_history_index
        }

        if let Ok(current_clipboard_content) = ctx.get_contents() {


            if clipboard_changed(
                &mut last_clipboard_content,
                &current_clipboard_content
            ) && !history_item_selected {

                if !history_item_selected {
                    last_clipboard_content = current_clipboard_content;
                    (*history_ref).push(
                        last_clipboard_content.to_owned()
                    );

                    println!("New history[{active_history_index}]: {history_ref:#?}");
                }
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

pub fn control_alt_number_pressed(
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

    let alt_pressed: bool = keys.contains(
        &Keycode::LAlt) || keys.contains(&Keycode::RAlt
    );

    let alt_and_control_pressed: bool = alt_pressed && control_pressed;

    for key in keys {
        match key {
            Keycode::Key1 | Keycode::Key2 |
            Keycode::Key3 | Keycode::Key4 |
            Keycode::Key5 | Keycode::Key6 |
            Keycode::Key7 | Keycode::Key8 |
            Keycode::Key9 | Keycode::Key0 => {

                if alt_and_control_pressed {

                    match number_mapping.get(key) {
                        Some(v) => {
                            return Some(v.clone())
                        },
                        None => {
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

pub fn keyup_event(
    current_keys: &HashSet<Keycode>,
    previous_keys: &HashSet<Keycode>
)  -> Option<Keycode> {

    let hs: Vec<Keycode> = current_keys
        .difference(previous_keys)
        .into_iter()
        .map(|&key| {key.clone()})
        .into_iter()
        .collect::<Vec<Keycode>>();

    match hs.get(0) {
        Some(value) => Some(value.clone()),
        None => None
    }
}

pub fn keydown_event(
    current_keys: &HashSet<Keycode>,
    previous_keys: &HashSet<Keycode>
) -> Option<Keycode> {

    let hs: Vec<Keycode> = previous_keys
        .difference(current_keys)
        .into_iter()
        .map(|&key| {key.clone()})
        .into_iter()
        .collect::<Vec<Keycode>>();

    match hs.get(0) {
        Some(value) => Some(value.clone()),
        None => None
    }
 

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
