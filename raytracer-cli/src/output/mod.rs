use image::{DynamicImage, Rgb32FImage};
use raytracer_lib::Framebuffer;

pub(crate) fn save(output_path: &str, fb: Framebuffer) -> Result<(), Box<dyn std::error::Error>> {
    let mut image_buffer = Rgb32FImage::new(fb.width, fb.height);

    const GAMMA: f32 = 2.2;
    // Get output pixels and convert from linear to sRGB gamma space
    fb.get_pixels()
        .into_iter()
        .map(|pixel| {
            let r = pixel[0].clamp(0., 1.).powf(1.0 / GAMMA);
            let g = pixel[1].clamp(0., 1.).powf(1.0 / GAMMA);
            let b = pixel[2].clamp(0., 1.).powf(1.0 / GAMMA);
            [r, g, b]
        })
        .enumerate()
        .for_each(|(idx, pixel)| {
            let x = idx as u32 % fb.width;
            let y = fb.width - 1 - idx as u32 / fb.width;
            image_buffer.put_pixel(x, y, pixel.into());
        });

    DynamicImage::from(image_buffer)
        .into_rgb16()
        .save(output_path)?;

    Ok(())
}
