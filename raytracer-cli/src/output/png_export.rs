pub(crate) fn save_to_png(output_path: &str, fb: &raytracer_lib::Framebuffer) {
    let height = fb.dimensions().1;
    let mut img = image::ImageBuffer::new(fb.dimensions().0, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let color = fb.get_pixel(x, height - y - 1);
        let r = (color[0].clamp(0.0, 1.0) * 255.0).round() as u8;
        let g = (color[1].clamp(0.0, 1.0) * 255.0).round() as u8;
        let b = (color[2].clamp(0.0, 1.0) * 255.0).round() as u8;

        *pixel = image::Rgb([r, g, b]);
    }

    img.save(output_path).unwrap();
}
