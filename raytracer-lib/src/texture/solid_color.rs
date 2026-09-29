use super::*;

#[derive(Debug)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn color(&self, _uv: (Real, Real), _p: &P3) -> Color {
        self.color
    }
}
