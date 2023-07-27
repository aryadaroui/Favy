// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{copy, remove_file};
use tauri::Manager;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PhotoFeedback {
    name: String,
    love: i8, // TODO: change this to sentiment
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

type FeedbackArray = Vec<PhotoFeedback>;

#[tauri::command]
async fn export(
    dir: String,
    photos: FeedbackArray,
    settings: ExportSettings,
) -> Result<(), String> {
    for photo in &photos {
        if photo.love == 1 && settings.heart {
            match copy(
                dir.clone() + &photo.name,
                dir.clone() + "love/" + &photo.name,
            ) {
                Ok(_) => {}
                Err(e) => return Err(format!("Failed to copy {} to ./love: {}", photo.name, e)),
            };
        }

        if photo.rating == 3 && settings.star3 {
            match copy(
                dir.clone() + &photo.name,
                dir.clone() + "star3/" + &photo.name,
            ) {
                Ok(_) => {}
                Err(e) => return Err(format!("Failed to copy {} to ./star3: {}", photo.name, e)),
            };
        }

        if photo.rating == 2 && settings.star2 {
            match copy(
                dir.clone() + &photo.name,
                dir.clone() + "star2/" + &photo.name,
            ) {
                Ok(_) => {}
                Err(e) => return Err(format!("Failed to copy {} to ./star2: {}", photo.name, e)),
            };
        }

        if photo.rating == 1 && settings.star1 {
            match copy(
                dir.clone() + &photo.name,
                dir.clone() + "star1/" + &photo.name,
            ) {
                Ok(_) => {}
                Err(e) => return Err(format!("Failed to copy {} to ./star1: {}", photo.name, e)),
            };
        }

        if photo.love == -1 && settings.hate {
            match copy(
                dir.clone() + &photo.name,
                dir.clone() + "hate/" + &photo.name,
            ) {
                Ok(_) => {}
                Err(e) => return Err(format!("Failed to copy {} to ./hate: {}", photo.name, e)),
            };
        }
    }

    if settings.delete_original {
        for photo in &photos {
            match remove_file(dir.clone() + &photo.name) {
                Ok(_) => {}
                Err(e) => return Err(format!("Failed to delete {}: {}", photo.name, e)),
            };
        }
    }

    Ok(())
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
