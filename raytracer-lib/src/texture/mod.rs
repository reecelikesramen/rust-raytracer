mod checkered;
mod image;
mod solid_color;

use std::fmt::Debug;

use crate::prelude::*;

pub use checkered::CheckeredTexture;
pub use image::ImageTexture;
pub use solid_color::SolidColor;

pub trait Texture: Send + Sync + Debug {
    fn color(&self, uv: (Real, Real), point: &P3) -> Color;
}
