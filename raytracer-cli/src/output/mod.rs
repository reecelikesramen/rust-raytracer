mod png_export;

use self::png_export::save_to_png;
use raytracer_lib::Framebuffer;
use std::path::Path;

pub(crate) fn save(output_path: &str, fb: Framebuffer) {
    const GAMMA: f32 = 2.2;
    // Get raw pixels and convert from linear to sRGB gamma space
    let dimensions = fb.dimensions();
    let pixels = fb
        .into_raw()
        .into_iter()
        .map(|pixel| {
            [
                pixel[0].powf(1.0 / GAMMA),
                pixel[1].powf(1.0 / GAMMA),
                pixel[2].powf(1.0 / GAMMA),
            ]
        })
        .collect::<Vec<_>>();

    let fb = Framebuffer::from_raw(dimensions.0, dimensions.1, pixels);

    if let Some(ext) = Path::new(output_path).extension() {
        if let Some(ext) = ext.to_str() {
            if ext == "png" {
                save_to_png(output_path, &fb);
            } else {
                unimplemented!("The format '{}' is not supported", ext)
            }
        }
    }
}
