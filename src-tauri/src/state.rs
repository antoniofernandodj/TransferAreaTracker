use std::sync::{Arc, Mutex};
use crate::pubsub::pubsub_models::PtHistoryGroup; // This line declares the `pubsub` module


#[derive(Clone)]
pub struct AppState {
    pub hg: PtHistoryGroup,
}

impl AppState {
    fn new(history_group: PtHistoryGroup) -> Self {
        AppState{hg: history_group}
    }
}

pub struct State {
    inner: Arc<Mutex<AppState>>,
}

impl State {
    pub fn new(history_group: PtHistoryGroup) -> Self {
        State {
            inner: Arc::new(Mutex::new(AppState::new(history_group))),
        }
    }

    pub fn get_state(&self) -> Arc<Mutex<AppState>> {
        self.inner.clone()
    }
}
