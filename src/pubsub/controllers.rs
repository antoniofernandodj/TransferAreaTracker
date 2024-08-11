use std::process::Command;
use super::models::{Event, PtHistoryGroup};

fn clear_screen() {
    let mut command: Command = Command::new("clear");
    let mut result: std::process::Child = command.spawn().unwrap();
    result.wait().unwrap();
}


pub fn history_new_item_in_history(event: &Event, history_group: &PtHistoryGroup) {
    
    if let Ok(mut hg) = history_group.lock() {
        clear_screen();

        let active = hg.get_active_history_number();
        let new_history_item = event.last_clipboard_content.clone()
        .expect("Novo item de histórico não encontrado!");
        
        println!("Novo item no historico!...");
        
        let active_history: &mut Vec<String> = hg.get_active_history();
        (*active_history).push(new_history_item);

        println!("Novo historico {}:", active);

        println!("{:#?}", active_history)
    }
}

#[allow(unused)]
pub fn key_up(event: &Event, history_group: &PtHistoryGroup) {
    // println!("Key Up!...");
    // println!("Evento: {event:?}")
}

#[allow(unused)]
pub fn key_down(event: &Event, history_group: &PtHistoryGroup) {
    // println!("Key down!...");
    // println!("Evento: {event:?}")
}

#[allow(unused)]
pub fn updated_previous_keys_set(event: &Event, history_group: &PtHistoryGroup) {
    // println!("updated_previous_keys_set!...");
    // println!("Evento: {event:?}")
}

pub fn history_selected(event: &Event, history_group: &PtHistoryGroup) {
    
    if let Ok(mut hg) = history_group.lock() {

        clear_screen();
        let history_number = event.history.expect("Valor não encontrado!");
        hg.set_active(history_number as i32);
        
        println!("Historico {} Selecionado!...", history_number);
        println!("Estado do history: {:#?}", hg.get_active_history());

    }
}
