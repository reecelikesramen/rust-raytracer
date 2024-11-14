use crate::color;
use crate::prelude::*;

use super::Shader;

#[derive(Debug, Default)]
pub struct NormalShader;

impl Shader for NormalShader {
    fn apply(&self, hit: &super::Hit) -> Color {
        let [x, y, z] = hit.normal.into();
        return color!(1.0 + x as f32, 1.0 + y as f32, 1.0 + z as f32) / 2.0;
    }
}
