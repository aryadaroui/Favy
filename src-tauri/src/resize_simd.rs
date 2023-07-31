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
	let mut img_buf: Vec<u8> = Vec::new();
	JpegEncoder::new(&mut img_buf)
		.write_image(img.as_raw(), img.width(), img.height(), ColorType::Rgb8)
		.unwrap();
	let img_base64 = general_purpose::STANDARD.encode(&img_buf);
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

fn orientation_from_file(data: &Vec<u8>) -> u32 {
	let mut cursor = std::io::Cursor::new(&data);
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

#[tauri::command(rename_all = "snake_case")]
pub async fn resize_simd(image_path: String) -> Result<String, String> {
	if is_jpeg(image_path.as_str()) {
		// if JPEG, use zune_jpeg which is faster
		// also don't do any alpha channel stuff
		let data: Vec<u8> = read(image_path).unwrap();

		let orientation = orientation_from_file(&data);
		let mut decoder = JpegDecoder::new(&data);
		let pixels = decoder.decode().unwrap();
		let info = decoder.info().unwrap();

		let src_image_jpeg = fr::Image::from_vec_u8(
			NonZeroU32::new(info.width.into()).unwrap(),
			NonZeroU32::new(info.height.into()).unwrap(),
			pixels,
			fr::PixelType::U8x3,
		)
		.unwrap();


		let scale_factor: f32 = (info.height as f32 / 256.0).into();

		let dst_height_jpeg = NonZeroU32::new(256).unwrap();
		let dst_width_jpeg = NonZeroU32::new((info.width as f32 / scale_factor) as u32).unwrap();

		let mut dst_image_jpeg = fr::Image::new(dst_width_jpeg, dst_height_jpeg, src_image_jpeg.pixel_type());
		let mut dst_view_jpeg = dst_image_jpeg.view_mut();

		let mut resizer_jpeg = fr::Resizer::new(fr::ResizeAlg::Nearest);
		resizer_jpeg.resize(&src_image_jpeg.view(), &mut dst_view_jpeg).unwrap();

		let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(
			dst_width_jpeg.get(),
			dst_height_jpeg.get(),
			dst_image_jpeg.into_vec(),
		)
		.unwrap();

		auto_rotate(&mut img, orientation); // rotate img if needed
		Ok(encode_jpeg_base64(&img))
	} else {
		// Read source image from file
		let img = ImageReader::open(image_path).unwrap().decode().unwrap();
		let width = NonZeroU32::new(img.width()).unwrap();
		let height = NonZeroU32::new(img.height()).unwrap();
		let src_image = fr::Image::from_vec_u8(width, height, img.to_rgba8().into_raw(), fr::PixelType::U8x4).unwrap();

		let scale_factor: f32 = (height.get() as f32 / 200.0).into();

		// Create container for data of destination image
		let dst_width = NonZeroU32::new(200).unwrap();
		let dst_height = NonZeroU32::new((width.get() as f32 / scale_factor) as u32).unwrap();
		let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

		// Get mutable view of destination image data
		let mut dst_view = dst_image.view_mut();

		// Create Resizer instance and resize source image
		// into buffer of destination image
		let mut resizer = fr::Resizer::new(fr::ResizeAlg::Nearest);
		resizer.resize(&src_image.view(), &mut dst_view).unwrap();

		let mut result_buf: Vec<u8> = Vec::new();
		JpegEncoder::new(&mut result_buf)
			.write_image(dst_image.buffer(), dst_width.get(), dst_height.get(), ColorType::Rgba8)
			.unwrap();
		let base64_str = general_purpose::STANDARD.encode(&result_buf);

		return Ok(base64_str);
	}
}
