use serde_json::json;
use shared_child::SharedChild;
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Read},
    os::windows::process::CommandExt,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};
use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder, WindowEvent};

#[cfg(target_os = "macos")]
use tauri::utils::TitleBarStyle;

pub mod utils;

#[derive(Default)]
struct AppState {
    ffmpeg_processes: HashMap<String, Arc<SharedChild>>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoParams {
    input_path: String,
    output_path: String,
    audio_codec: Option<String>,
    video_codec: Option<String>,
}

#[tauri::command]
fn is_ffmpeg_available() -> Result<bool, String> {
    Command::new("ffmpeg")
        .arg("-version")
        .creation_flags(utils::CREATE_NO_WINDOW)
        .output()
        .map(|output| output.status.success())
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn compress(
    params: VideoParams,
    app: AppHandle,
    app_state: State<'_, Mutex<AppState>>,
) -> Result<bool, String> {
    let state = app_state.lock().unwrap();
    if state.ffmpeg_processes.contains_key(&params.output_path) {
        return Ok(false);
    }
    drop(state);

    let duration = utils::get_video_duration(&params.input_path);

    let mut command = Command::new("ffmpeg");
    command.arg("-y").arg("-i").arg(&params.input_path);

    if let Some(video_codec) = params.video_codec {
        command.arg("-c:v").arg(video_codec);
    }

    if let Some(audio_codec) = params.audio_codec {
        command.arg("-c:a").arg(audio_codec);
    }

    command
        .arg("-progress")
        .arg("pipe:1")
        .arg("-loglevel")
        .arg("error")
        .arg(&params.output_path)
        .creation_flags(utils::CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let shared_child = SharedChild::spawn(&mut command).unwrap();
    let child_arc = Arc::new(shared_child);

    let stdout = child_arc.take_stdout().unwrap();
    let stderr = child_arc.take_stderr().unwrap();

    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("out_time=") {
                    if let Some(out_time) = line.split('=').nth(1) {
                        if let Some(out_time) = utils::parse_time(out_time) {
                            let payload = json!({
                                "filePath": params.input_path,
                                "percentage": (out_time / duration) * 100.0
                            });
                            app.emit("compress:progress", payload.to_string()).unwrap();
                        }
                    }
                }
            }
        }
    });

    let error = Arc::new(Mutex::new(String::new()));
    let error_clone = error.clone();

    thread::spawn(move || {
        let mut error = error_clone.lock().unwrap();
        let mut reader = BufReader::new(stderr);
        reader.read_to_string(&mut *error).unwrap();
    });

    let mut state = app_state.lock().unwrap();
    state
        .ffmpeg_processes
        .insert(params.output_path.clone(), child_arc.clone());
    drop(state);

    let exit_status = child_arc.wait().unwrap();
    let mut state = app_state.lock().unwrap();
    state.ffmpeg_processes.remove(&params.output_path);

    let error = error.lock().unwrap();
    if !error.is_empty() {
        return Err(error.clone());
    }

    Ok(exit_status.success())
}

#[tauri::command]
async fn cancel_compress(
    output_path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<bool, String> {
    let mut state = state.lock().map_err(|_| "Failed to lock state")?;
    if let Some(child) = state.ffmpeg_processes.remove(&output_path) {
        if let Err(e) = child.kill() {
            return Err(format!("Failed to kill child: {}", e));
        }

        thread::sleep(std::time::Duration::from_secs(1));
        if fs::metadata(&output_path).is_ok() {
            fs::remove_file(&output_path).map_err(|e| e.to_string())?;
        }
    }
    Ok(true)
}

#[tauri::command]
fn get_file_size(path: String) -> Result<u64, String> {
    fs::metadata(path)
        .map(|metadata| metadata.len())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn show_in_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", &path]) // comma is important
            .creation_flags(utils::CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn close_request(app: AppHandle, state: State<'_, Mutex<AppState>>) {
    let state = state.lock().unwrap();
    for (output_path, child) in state.ffmpeg_processes.iter() {
        child.kill().expect("failed to kill child");
        thread::sleep(std::time::Duration::from_secs(1));
        if fs::metadata(output_path).is_ok() {
            fs::remove_file(output_path).unwrap();
        }
    }
    app.exit(0);
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            is_ffmpeg_available,
            compress,
            cancel_compress,
            get_file_size,
            show_in_folder,
            close_request,
        ])
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));

            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("simpleff")
                .inner_size(360.0, 420.0)
                .resizable(false)
                .fullscreen(false)
                .decorations(false)
                .shadow(false)
                .transparent(true);

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

            Ok(())
        })
        .on_window_event(|window, event| match event {
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
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
