use serde::{Deserialize, Serialize};
use std::fs;
use trash;

struct SubDirs {
	love: &'static str,
	star3: &'static str,
	star2: &'static str,
	star1: &'static str,
	dislike: &'static str,
}

const SUB_DIRS: SubDirs = SubDirs {
	love: "love",
	star3: "star3",
	star2: "star2",
	star1: "star1",
	dislike: "dislike",
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoFeedback {
	name: String,
	love: i8, // TODO: change this to sentiment
	rating: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportSettings {
	favorite: bool,
	star3: bool,
	star2: bool,
	star1: bool,
	dislike: bool,
	delete_original: bool,
}

pub type FeedbackArray = Vec<PhotoFeedback>;

fn create_dir(setting: bool, dir: String) -> Result<(), String> {
	if setting {
		match fs::create_dir_all(dir.clone()) {
			Ok(_) => Ok(()),
			Err(e) => Err(format!("Failed to create {}: {}", dir, e)),
		}
	} else {
		Ok(())
	}
}

fn copy_photo(condition: bool, dir: String, sub_dir: &str, photo: PhotoFeedback) -> Result<(), String> {
	if condition {
		match fs::copy(dir.clone() + &photo.name, dir.clone() + sub_dir + "/" + &photo.name) {
			Ok(_) => Ok(()),
			Err(e) => Err(format!("Failed to copy {} to ./{}: {}", photo.name, sub_dir, e)),
		}
	} else {
		Ok(())
	}
}

#[tauri::command]
pub async fn export(dir: String, photos: FeedbackArray, settings: ExportSettings) -> Result<(), String> {
	// create directories if they don't exist
	create_dir(settings.favorite, dir.clone() + SUB_DIRS.love)?;
	create_dir(settings.star3, dir.clone() + SUB_DIRS.star3)?;
	create_dir(settings.star2, dir.clone() + SUB_DIRS.star2)?;
	create_dir(settings.star1, dir.clone() + SUB_DIRS.star1)?;
	create_dir(settings.dislike, dir.clone() + SUB_DIRS.dislike)?;

	// copy photos to their respective directories
	for photo in &photos {
		copy_photo(photo.love == 1 && settings.favorite, dir.clone(), SUB_DIRS.love, photo.clone())?;
		copy_photo(photo.rating == 3 && settings.star3, dir.clone(), SUB_DIRS.star3, photo.clone())?;
		copy_photo(photo.rating == 2 && settings.star2, dir.clone(), SUB_DIRS.star2, photo.clone())?;
		copy_photo(photo.rating == 1 && settings.star1, dir.clone(), SUB_DIRS.star1, photo.clone())?;
		copy_photo(photo.love == -1 && settings.dislike, dir.clone(), SUB_DIRS.dislike, photo.clone())?;
	}

	if settings.delete_original {
		for photo in &photos {
			let trashable = (photo.love == 1 && settings.favorite)
				|| (photo.rating == 3 && settings.star3)
				|| (photo.rating == 2 && settings.star2)
				|| (photo.rating == 1 && settings.star1)
				|| (photo.love == -1 && settings.dislike);

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
