// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use tauri::Manager;
use trash;
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
    favorite: bool,
    star3: bool,
    star2: bool,
    star1: bool,
    disliked: bool,
    delete_original: bool,
}

type FeedbackArray = Vec<PhotoFeedback>;

#[tauri::command]
async fn export(
    dir: String,
    photos: FeedbackArray,
    settings: ExportSettings,
) -> Result<(), String> {
    // create directories if they don't exist
    if settings.favorite {
        match fs::create_dir_all(dir.clone() + "love") {
            Ok(_) => {}
            Err(e) => return Err(format!("Failed to create ./love: {}", e)),
        };
    }

    if settings.star3 {
        match fs::create_dir_all(dir.clone() + "star3") {
            Ok(_) => {}
            Err(e) => return Err(format!("Failed to create ./star3: {}", e)),
        };
    }

    if settings.star2 {
        match fs::create_dir_all(dir.clone() + "star2") {
            Ok(_) => {}
            Err(e) => return Err(format!("Failed to create ./star2: {}", e)),
        };
    }

    if settings.star1 {
        match fs::create_dir_all(dir.clone() + "star1") {
            Ok(_) => {}
            Err(e) => return Err(format!("Failed to create ./star1: {}", e)),
        };
    }

    if settings.disliked {
        match fs::create_dir_all(dir.clone() + "hate") {
            Ok(_) => {}
            Err(e) => return Err(format!("Failed to create ./hate: {}", e)),
        };
    }

    // copy photos to their respective directories
    for photo in &photos {
        if photo.love == 1 && settings.favorite {
            match fs::copy(
                dir.clone() + &photo.name,
                dir.clone() + "love/" + &photo.name,
            ) {
                Ok(_) => {}
                Err(e) => return Err(format!("Failed to copy {} to ./love: {}", photo.name, e)),
            };
        }

        if photo.rating == 3 && settings.star3 {
            match fs::copy(
                dir.clone() + &photo.name,
                dir.clone() + "star3/" + &photo.name,
            ) {
                Ok(_) => {}
                Err(e) => return Err(format!("Failed to copy {} to ./star3: {}", photo.name, e)),
            };
        }

        if photo.rating == 2 && settings.star2 {
            match fs::copy(
                dir.clone() + &photo.name,
                dir.clone() + "star2/" + &photo.name,
            ) {
                Ok(_) => {}
                Err(e) => return Err(format!("Failed to copy {} to ./star2: {}", photo.name, e)),
            };
        }

        if photo.rating == 1 && settings.star1 {
            match fs::copy(
                dir.clone() + &photo.name,
                dir.clone() + "star1/" + &photo.name,
            ) {
                Ok(_) => {}
                Err(e) => return Err(format!("Failed to copy {} to ./star1: {}", photo.name, e)),
            };
        }

        if photo.love == -1 && settings.disliked {
            match fs::copy(
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
            let trashable = (photo.love == 1 && settings.favorite)
                || (photo.rating == 3 && settings.star3)
                || (photo.rating == 2 && settings.star2)
                || (photo.rating == 1 && settings.star1)
                || (photo.love == -1 && settings.disliked);

            if trashable {
                match trash::delete(dir.clone() + &photo.name) {
                    Ok(_) => {}
                    Err(e) => return Err(format!("Failed to move {} to trash: {}", photo.name, e)),
                }
            }
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
