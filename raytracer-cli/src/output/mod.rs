mod png_export;

use self::png_export::save_to_png;
use raytracer_lib::Framebuffer;
use std::path::Path;

pub(crate) fn save(output_path: &str, mut fb: Framebuffer) {
    const GAMMA: f32 = 2.2;
    // convert from linear to sRGB gamma space
    fb.pixels = fb
        .pixels
        .iter_mut()
        .map(|pixel| pixel.map(|c| c.powf(1.0 / GAMMA)))
        .collect();

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
