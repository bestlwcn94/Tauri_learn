// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_docs(handle: tauri::AppHandle) {
   tauri::WindowBuilder::new(
        &handle,
        "lable2", /* the unique window label */
        tauri::WindowUrl::App("#/Greet1".into()),
    )
    .build().unwrap();
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,open_docs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
