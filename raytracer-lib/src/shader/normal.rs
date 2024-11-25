use crate::color;
use crate::prelude::*;

use super::Shader;

#[derive(Debug, Default)]
pub struct NormalShader;

impl Shader for NormalShader {
    fn apply(&self, hit: &super::HitRecord) -> Color {
        panic!("unimplemented");
        // return color!(
        //     1.0 + hit.normal.x as f32,
        //     1.0 + hit.normal.y as f32,
        //     1.0 + hit.normal.z as f32
        // ) / 2.0;
    }
}
