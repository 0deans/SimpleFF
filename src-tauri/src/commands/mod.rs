use crate::state::AppState;
use std::{fs, sync::Mutex, thread};
use tauri::{AppHandle, State};

pub mod ffmpeg;

#[tauri::command]
pub fn close_request(app: AppHandle, state: State<'_, Mutex<AppState>>) {
    let processes = {
        let mut state = state.lock().expect("State lock poisoned");
        state.ffmpeg_processes.drain().collect::<Vec<_>>()
    };

    let handles: Vec<_> = processes
        .into_iter()
        .map(|(output_path, child)| {
            thread::spawn(move || {
                if let Err(e) = child.kill() {
                    eprintln!("Failed to kill ffmpeg process: {}", e);
                }

                let _ = child.wait();

                if let Err(e) = fs::remove_file(&output_path) {
                    if e.kind() != std::io::ErrorKind::NotFound {
                        eprintln!("Failed to delete: {}", e);
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Failed to join cleanup thread");
    }

    app.exit(0);
}
