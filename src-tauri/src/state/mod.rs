use shared_child::SharedChild;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct AppState {
    pub ffmpeg_processes: HashMap<String, Arc<SharedChild>>,
}

pub fn app_state() -> Mutex<AppState> {
    Mutex::new(AppState::default())
}
