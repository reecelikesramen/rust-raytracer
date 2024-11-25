use crate::prelude::*;

#[derive(Debug, Default)]
pub struct NullShader;

impl super::Shader for NullShader {
    fn apply(&self, _hit: &super::HitRecord) -> Color {
        ERROR_COLOR
    }
}
