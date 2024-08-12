// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod pubsub;
pub mod state;

use tokio::time::sleep;
use tauri::async_runtime;
use std::{
    sync::{
        Arc,
        Mutex
    }, time::Duration
};

use pubsub::{
    pubsub_init,
    pubsub_history_group::HistoryGroup,
    pubsub_models::PtHistoryGroup
};

static LOOP_TIME: u64 = 50;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn send_event(
    state: tauri::State<'_, state::State>,
    window: tauri::Window,
) -> Result<(), String> {

    loop {

        sleep(Duration::from_millis(LOOP_TIME)).await;

        if let Ok(app_state) = state.get_state().lock() {
            if let Ok(mut hg) = app_state.hg.lock() {
                
                let active_history = hg.get_active_history().clone();
                let active_history_number = hg.get_active_history_number();

                if let Err(e) = window.emit("active-history", &active_history) {
                    return Err(format!("Failed to emit event: {}", e));
                } else {
                    println!("Event Emitted!, {:?}", active_history);
                }

                if let Err(e) = window.emit("active-history-number", active_history_number) {
                    return Err(format!("Failed to emit event: {}", e));
                } else {
                    println!("Event Emitted!, {:?}", active_history_number);
                }
            }
        } else {
            return Err("Failed to lock app state".into());
        }

    }

}


#[tauri::command]
fn set_history(n: i32, state: tauri::State<'_, state::State>) -> Vec<String> {

    // println!("Changing to history {}", n);

    if let Ok(app_state) = state.get_state().lock() {
        if let Ok(mut hg) = app_state.hg.lock() {
            hg.set_active(n);

            // println!("This history {:?}", hg.get_active_history());
        }
    }

    Vec::new()
}


fn main() {

    let history_group: PtHistoryGroup = Arc::new(
        Mutex::new(HistoryGroup::new())
    );

    let (   history_group,
            mut publisher,
            mut consumer    ) = pubsub_init::init_pubsub(history_group);

    let _ = async_runtime::spawn(
        async move { publisher.run() }
    );

    let _ = async_runtime::spawn(
        async move { consumer.run(); }
    );

    let state = state::State::new(history_group);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            send_event,
            set_history
        ])
        .manage(state)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
