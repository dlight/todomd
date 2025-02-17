use std::sync::OnceLock;

static NUM: OnceLock<u32> = OnceLock::new();

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_num() -> u32 {
    *NUM.get().unwrap_or(&8)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(n: u32) {
    NUM.set(n).unwrap();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_num])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
