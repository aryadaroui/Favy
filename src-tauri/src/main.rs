// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PhotoFeedback {
    name: String,
    love: i8,
    rating: i8,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExportSettings {
    heart: bool,
    star3: bool,
    star2: bool,
    star1: bool,
    hate: bool,
    delete_original: bool,
}

// kind of redundant with Result, but I'm leaving it like this in case
// I have more statuses I want to add later
#[derive(Serialize)]
enum TaskResult {
    Done,
    Failed,
}

type FeedbackArray = Vec<PhotoFeedback>;

#[tauri::command]
async fn export(dir: String, photos: FeedbackArray, settings: ExportSettings, ) -> Result<TaskResult, String> {
    println!("Exporting...\n");



    // print all the export settings
    println!("Export Settings:");
    println!("- Heart: {}", settings.heart);
    println!("- Star 3: {}", settings.star3);
    println!("- Star 2: {}", settings.star2);
    println!("- Star 1: {}", settings.star1);
    println!("- Hate: {}", settings.hate);
    println!("- Delete Original: {}\n", settings.delete_original);

    // iterate through photos
    for photo in &photos {
        println!("Filename: {}", photo.name);
        println!("- Love: {}, Rating: {}\n", photo.love, photo.rating);
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
