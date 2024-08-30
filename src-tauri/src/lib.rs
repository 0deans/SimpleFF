use std::process::Command;

#[tauri::command]
fn is_ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[tauri::command]
fn compress(input_path: &str, output_path: &str) -> bool {
    println!("Compressing {} to {}", input_path, output_path);
    Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-c:v")
        .arg("libx264")
        .arg("-c:a")
        .arg("aac")
        .arg("-b:a")
        .arg("128k")
        .arg("-crf")
        .arg("26")
        .arg(output_path)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![is_ffmpeg_available, compress])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
