use base64::{engine::general_purpose, Engine as _};
use exif::{In, Tag};
use fast_image_resize as fr;
use image::codecs::jpeg::JpegEncoder;
use image::imageops;
use image::io::Reader as ImageReader;
use image::{ColorType, ImageBuffer, ImageEncoder, Rgb};
use std::fs::read;
use std::num::NonZeroU32;
use zune_jpeg::JpegDecoder; // TODO

fn is_jpeg(path: &str) -> bool {
	path.ends_with(".jpg") || path.ends_with(".jpeg") || path.ends_with(".JPG") || path.ends_with(".JPEG")
}

fn encode_jpeg_base64(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> String {
	let mut jpeg_data: Vec<u8> = Vec::new();
	JpegEncoder::new(&mut jpeg_data)
		.write_image(img.as_raw(), img.width(), img.height(), ColorType::Rgb8)
		.unwrap();
	let img_base64 = general_purpose::STANDARD.encode(&jpeg_data);
	return img_base64;
}

fn auto_rotate(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, orientation: u32) {
	match orientation {
		1 => (),
		2 => *img = imageops::flip_horizontal(img),
		3 => *img = imageops::rotate180(img),
		4 => *img = imageops::flip_vertical(img),
		5 => *img = imageops::flip_horizontal(&imageops::rotate90(img)),
		6 => *img = imageops::rotate90(img),
		7 => *img = imageops::flip_horizontal(&imageops::rotate270(img)),
		8 => *img = imageops::rotate270(img),
		_ => (),
	}
}

fn orientation_from_file(file_data: &Vec<u8>) -> u32 {
	let mut cursor = std::io::Cursor::new(&file_data);
	let exifreader = exif::Reader::new();
	let exif = exifreader.read_from_container(&mut cursor).unwrap();

	let orientation = match exif.get_field(Tag::Orientation, In::PRIMARY) {
		Some(orientation) => match orientation.value.get_uint(0) {
			Some(v @ 1..=8) => v,
			_ => 1,
		},
		None => 1,
	};

	return orientation;
}

fn resized_dims(width: u32, height: u32, max_dim: u32) -> (u32, u32) {
	let aspect_ratio = width as f32 / height as f32;
	if width > height {
		let new_width = max_dim;
		let new_height = (max_dim as f32 / aspect_ratio) as u32;
		(new_width, new_height)
	} else {
		let new_height = max_dim;
		let new_width = (max_dim as f32 * aspect_ratio) as u32;
		(new_width, new_height)
	}
}

#[tauri::command(rename_all = "snake_case")]
pub async fn resize_simd(image_path: String) -> Result<String, String> {
	if is_jpeg(image_path.as_str()) {
		// if JPEG, we zune_jpeg which is a faster decoer.
		// We also read the EXIF to orient the image properly.

		// Iniitalize
		let file_data: Vec<u8> = read(image_path).unwrap();
		let orientation = orientation_from_file(&file_data);
		let mut decoder = JpegDecoder::new(&file_data);
		let mut fir_resizer = fr::Resizer::new(fr::ResizeAlg::Nearest);

		// Decoder data
		let img_src = decoder.decode().unwrap();
		let img_src_info = decoder.info().unwrap();

		// dst dims
		let (dst_width, dst_height) = resized_dims(img_src_info.width.into(), img_src_info.height.into(), 256); // TODO: set parameter for max_dim

		// set up src and dst images for fir (fast_image_resize)
		let img_fir_src = fr::Image::from_vec_u8(
			NonZeroU32::new(img_src_info.width.into()).unwrap(),
			NonZeroU32::new(img_src_info.height.into()).unwrap(),
			img_src,
			fr::PixelType::U8x3,
		)
		.unwrap();
		let mut img_fir_dst = fr::Image::new(
			NonZeroU32::new(dst_width).unwrap(),
			NonZeroU32::new(dst_height).unwrap(),
			img_fir_src.pixel_type(),
		);

		// resize
		fir_resizer
			.resize(&img_fir_src.view(), &mut img_fir_dst.view_mut())
			.unwrap();

		// convert to rotatable image type
		let mut img_buffer =
			ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(dst_width, dst_height, img_fir_dst.into_vec()).unwrap();
		auto_rotate(&mut img_buffer, orientation); // rotate img if needed
		Ok(encode_jpeg_base64(&img_buffer)) // encode jpeg then to base64
	} else {
		// if not JPEG, we use image crate to decode format dynamically
		// This does NOT handle orientation.
		// this will also squish the alpha channel to black if it exists.

		// Read source image from file
		let src_img = ImageReader::open(image_path).unwrap().decode().unwrap();

		// dst dims
		let (dst_width, dst_height) = resized_dims(src_img.width(), src_img.height(), 256); // TODO: set parameter for max_dim

		// set up src and dst images for fir (fast_image_resize)
		let img_fir_src = fr::Image::from_vec_u8(
			NonZeroU32::new(src_img.width()).unwrap(),
			NonZeroU32::new(src_img.height()).unwrap(),
			src_img.to_rgba8().into_raw(),
			fr::PixelType::U8x4,
		)
		.unwrap();
		let mut img_fir_dst = fr::Image::new(
			NonZeroU32::new(dst_width).unwrap(),
			NonZeroU32::new(dst_height).unwrap(),
			img_fir_src.pixel_type(),
		);

		// Create Resizer instance and resize source image
		// into buffer of destination image
		let mut fir_resizer = fr::Resizer::new(fr::ResizeAlg::Nearest);
		fir_resizer
			.resize(&img_fir_src.view(), &mut img_fir_dst.view_mut())
			.unwrap();

		// TODO: consolidate this with encode_jpeg_base64()
		// This work for now, but I'd like to use the same function for both JPEG and non-JPEG
		// IDK how to manage the Rgba8 vs Rgb8 types, as I get corrupted images with ImageBuffer<Rgba<u8>, Vec<u8>>

		// Encode image to JPEG and convert to base64
		let mut img_buffer: Vec<u8> = Vec::new();
		JpegEncoder::new(&mut img_buffer)
			.write_image(img_fir_dst.buffer(), dst_width, dst_height, ColorType::Rgba8)
			.unwrap();
		let base64_str = general_purpose::STANDARD.encode(&img_buffer);

		Ok(base64_str)
	}
}
