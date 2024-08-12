use super::pubsub_models::{Event, PtHistoryGroup};


pub fn new_item_in_history(event: &Event, history_group: &PtHistoryGroup) {
    if let Ok(mut hg) = history_group.lock() {
        // clear_screen();

        let active: i32 = hg.get_active_history_number();
        let new_history_item: String = event.last_clipboard_content
            .clone()
            .expect("Novo item de histórico não encontrado!");

        println!("Novo item no historico!...");

        let active_history = hg.get_active_history();
        active_history.push(new_history_item);

        println!("Novo historico {}:", active);
        println!("{:#?}", active_history)
    }
}


pub fn history_selected(event: &Event, history_group: &PtHistoryGroup) {
    
    if let Ok(mut hg) = history_group.lock() {

        // clear_screen();
        let history_number = event.history.expect("Valor não encontrado!");
        hg.set_active(history_number as i32);
        
        println!("Historico {} Selecionado!...", history_number);
        println!("Estado do history: {:#?}", hg.get_active_history());

    }
}
