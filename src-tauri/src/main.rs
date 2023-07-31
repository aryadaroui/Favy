// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate exif;

mod export;
mod make_thumb;

use tauri::Manager;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

fn main() {
	tauri::Builder::default()
		.setup(|app| {
			let window = app.get_window("main").unwrap();

			#[cfg(target_os = "macos")]
			apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, Some(16.0))
				.expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

			Ok(())
		})
		.invoke_handler(tauri::generate_handler![export::export, make_thumb::make_thumb])
		.run(tauri::generate_context!())
		.expect("Fatal error while running Favy.");
}
