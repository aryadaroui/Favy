use std::num::NonZeroU32;

use fast_image_resize as fr;
use image::codecs::jpeg::JpegEncoder; // change this to zune iamge in the future
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};
use zune_jpeg::JpegDecoder; // TODO

use std::fs::read;

use base64::{engine::general_purpose, Engine as _};

fn is_jpeg(path: &str) -> bool {
	path.ends_with(".jpg") || path.ends_with(".jpeg") || path.ends_with(".JPG") || path.ends_with(".JPEG")
}

#[tauri::command(rename_all = "snake_case")]
pub async fn resize_simd(image_path: String) -> Result<String, String> {
	if is_jpeg(image_path.as_str()) {
		// if JPEG, use zune_jpeg which is faster
		// also don't do any alpha channel stuff

		let data: Vec<u8> = read(image_path).unwrap();
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

		// TODO: account for orientation. e.g. use width instead of height if 90 or 270
		let scale_factor: f32 = (info.height as f32 / 200.0).into();

		let dst_height_jpeg = NonZeroU32::new(200).unwrap();
		let dst_width_jpeg = NonZeroU32::new((info.width as f32 / scale_factor) as u32).unwrap();

		let mut dst_image_jpeg = fr::Image::new(dst_width_jpeg, dst_height_jpeg, src_image_jpeg.pixel_type());
		let mut dst_view_jpeg = dst_image_jpeg.view_mut();

		let mut resizer_jpeg = fr::Resizer::new(fr::ResizeAlg::Nearest);
		resizer_jpeg.resize(&src_image_jpeg.view(), &mut dst_view_jpeg).unwrap();

		let mut result_buf_jpeg: Vec<u8> = Vec::new();
		JpegEncoder::new(&mut result_buf_jpeg)
			.write_image(dst_image_jpeg.buffer(), dst_width_jpeg.get(), dst_height_jpeg.get(), ColorType::Rgb8)
			.unwrap();
		let base64_str_jpeg = general_purpose::STANDARD.encode(&result_buf_jpeg);

		return Ok(base64_str_jpeg);
	} else {
		// Read source image from file
		let img = ImageReader::open(image_path).unwrap().decode().unwrap();
		let width = NonZeroU32::new(img.width()).unwrap();
		let height = NonZeroU32::new(img.height()).unwrap();
		let mut src_image =
			fr::Image::from_vec_u8(width, height, img.to_rgba8().into_raw(), fr::PixelType::U8x4).unwrap();

		// // Multiple RGB channels of source image by alpha channel
		// // (not required for the Nearest algorithm)
		let alpha_mul_div = fr::MulDiv::default();
		alpha_mul_div.multiply_alpha_inplace(&mut src_image.view_mut()).unwrap();


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

		// Divide RGB channels of destination image by alpha
		alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

		let mut result_buf: Vec<u8> = Vec::new();
		JpegEncoder::new(&mut result_buf)
			.write_image(dst_image.buffer(), dst_width.get(), dst_height.get(), ColorType::Rgba8)
			.unwrap();
		let base64_str = general_purpose::STANDARD.encode(&result_buf);

		return Ok(base64_str);
	}
}
