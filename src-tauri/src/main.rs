// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::state::AppState;
use std::sync::Mutex;
#[cfg(target_os = "macos")]
use tauri::utils::TitleBarStyle;
use tauri::{Emitter, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};

pub mod commands;
pub mod models;
pub mod state;
pub mod utils;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::ffmpeg::is_ffmpeg_available,
            commands::ffmpeg::process_video,
            commands::ffmpeg::cancel_video_processing,
            commands::close_request,
        ])
        .setup(|app| {
            app.manage(state::app_state());
            setup_window(app);
            Ok(())
        })
        .on_window_event(window_event_handler)
        .run(tauri::generate_context!())
        .expect("Failed to run application");
}

fn setup_window(app: &tauri::App) {
    let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("simpleff")
        .inner_size(360.0, 420.0)
        .resizable(false)
        .fullscreen(false)
        .decorations(false)
        .shadow(false);

    #[cfg(not(target_os = "macos"))]
    let win_builder = win_builder.transparent(true);

    #[cfg(target_os = "macos")]
    let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

    #[allow(unused_variables)]
    let window = win_builder.build().unwrap();

    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::{NSColor, NSWindow};
        use cocoa::base::{id, nil};

        let ns_window = window.ns_window().unwrap() as id;
        unsafe {
            let bg_color = NSColor::colorWithRed_green_blue_alpha_(nil, 1.0, 1.0, 1.0, 1.0);
            ns_window.setBackgroundColor_(bg_color);
        }
    }
}

fn window_event_handler(window: &tauri::Window, event: &tauri::WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            let app_handle = window.app_handle();
            let state = app_handle.state::<Mutex<AppState>>();
            let state = state.lock().unwrap();

            if !state.ffmpeg_processes.is_empty() {
                api.prevent_close();
                app_handle.emit("close-requested", ()).unwrap();
            }
        }
        _ => {}
    }
}
