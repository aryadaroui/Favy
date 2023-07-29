use std::num::NonZeroU32;

// use zune_jpeg::JpegDecoder; // TODO
use fast_image_resize as fr;
use image::codecs::jpeg::JpegEncoder; // change this to zune iamge in the future
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};

use base64::{engine::general_purpose, Engine as _};

#[tauri::command(rename_all = "snake_case")]
pub async fn resize_simd(image_path: String) -> Result<String, String> {
	// Read source image from file
	let img = ImageReader::open(image_path).unwrap().decode().unwrap();
	let width = NonZeroU32::new(img.width()).unwrap();
	let height = NonZeroU32::new(img.height()).unwrap();
	let mut src_image = fr::Image::from_vec_u8(width, height, img.to_rgba8().into_raw(), fr::PixelType::U8x4).unwrap();

	// Multiple RGB channels of source image by alpha channel
	// (not required for the Nearest algorithm)
	let alpha_mul_div = fr::MulDiv::default();
	alpha_mul_div.multiply_alpha_inplace(&mut src_image.view_mut()).unwrap();

	// Create container for data of destination image
	let dst_width = NonZeroU32::new(150).unwrap();
	let dst_height = NonZeroU32::new(100).unwrap();
	let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

	// Get mutable view of destination image data
	let mut dst_view = dst_image.view_mut();

	// Create Resizer instance and resize source image
	// into buffer of destination image
	let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
	resizer.resize(&src_image.view(), &mut dst_view).unwrap();

	// Divide RGB channels of destination image by alpha
	alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

	let result_buf = Vec::new();
	JpegEncoder::new(result_buf)
		.write_image(dst_image.buffer(), dst_width.get(), dst_height.get(), ColorType::Rgba8)
		.unwrap();

	let mut result_buf = Vec::new();
	JpegEncoder::new(&mut result_buf)
		.write_image(dst_image.buffer(), dst_width.get(), dst_height.get(), ColorType::Rgba8)
		.unwrap();
	let base64_str = general_purpose::STANDARD.encode(&result_buf);

	Ok(base64_str)
}
