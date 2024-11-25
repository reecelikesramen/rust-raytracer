use std::sync::Arc;

use ::image::Rgb32FImage;

use crate::color;

use super::*;

#[derive(Debug)]
pub struct ImageTexture {
    data: Arc<Rgb32FImage>,
    tint: Color,
}

impl ImageTexture {
    pub fn new(data: Arc<Rgb32FImage>, tint: Color) -> Self {
        Self { data, tint }
    }
}

impl Texture for ImageTexture {
    fn color(&self, uv: (Real, Real), _p: &P3) -> Color {
        // clamp u and v
        let u = uv.0.clamp(0., 1.);
        // flip v coordinate since image coordinates are inverted
        let v = 1. - uv.1.clamp(0., 1.);

        let i = (u * self.data.width() as Real) as u32;
        let j = (v * self.data.height() as Real) as u32;

        let pixel = self.data.get_pixel(i, j);

        color!(pixel[0], pixel[1], pixel[2]).component_mul(&self.tint)
    }
}
