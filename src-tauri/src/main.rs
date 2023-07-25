// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    love: i8,
    rating: i8,
}

#[derive(Serialize)]
enum TaskResult {
    Done,
    Failed,
}

type FileNameHashMap = HashMap<String, Data>;

#[tauri::command]
async fn export(photos: FileNameHashMap) -> Result<TaskResult, String> {
    println!("Exporting...\n");

    for (key, value) in &photos {
        println!("Filename: {}", key);
        println!("- Love: {}, Rating: {}\n", value.love, value.rating);
    }
    Ok(TaskResult::Done)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, Some(16.0))
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![export])
        .run(tauri::generate_context!())
        .expect("Fatal error while running Favy.");
}
