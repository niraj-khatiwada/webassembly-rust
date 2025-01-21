use image::{DynamicImage, GenericImageView};
use imagequant::RGBA;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn compress(raw_blob: &str, quality: u8) -> String {
    log(&"New image received. Compression started."
        .to_string()
        .into());
    const BASE_64_PREFIX: &str = "data:image/png;base64,";
    let blob = if raw_blob.starts_with(BASE_64_PREFIX) {
        let strip_size = BASE_64_PREFIX.len();
        &raw_blob[(strip_size)..]
    } else {
        raw_blob
    };
    let decoded_image = base64::decode(blob).expect("Failed to decode base64.");
    let img = image::load_from_memory(&decoded_image).expect("Failed to load image.");
    let compressed_image = compress_image(img, quality).expect("Failed to compress the image.");

    let mut buffer = Vec::new();
    compressed_image
        .write_to(&mut buffer, image::ImageOutputFormat::Png)
        .expect("Could not convert to a buffer");
    let base64_encoded = base64::encode(&buffer);
    let data_url = format!("{BASE_64_PREFIX}{}", base64_encoded);
    log(&"Compression complete.".to_string().into());
    data_url
}

fn compress_image(
    img: DynamicImage,
    quality: u8,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let (w, h) = img.dimensions();

    let width = w as usize;
    let height = h as usize;

    let img_rgba8 = img.to_rgba8();

    let rgba_pixels: Vec<RGBA> = img_rgba8
        .pixels()
        .map(|p| RGBA {
            r: p[0],
            g: p[1],
            b: p[2],
            a: p[3],
        })
        .collect();

    let mut attr = imagequant::new();

    attr.set_quality(0, quality).unwrap();

    let mut liq_image = attr.new_image(rgba_pixels, width, height, 0.0)?;

    attr.set_max_colors(256).unwrap();

    let mut result = attr.quantize(&mut liq_image)?;

    let (palette, pixels) = result.remapped(&mut liq_image)?;

    let mut quantized_img = image::ImageBuffer::new(w, h);

    for (x, y, pixel) in quantized_img.enumerate_pixels_mut() {
        let idx = (y * w + x) as usize;
        let p = &palette[pixels[idx] as usize];
        *pixel = image::Rgba([p.r, p.g, p.b, p.a]);
    }

    Ok(DynamicImage::ImageRgba8(quantized_img))
}
