use crate::{color, prelude::*};

#[derive(Debug, Default)]
pub struct NullShader;

static ERROR_COLOR: Color = color!(1.0, 0.0, 1.0);

impl super::Shader for NullShader {
    fn apply(&self, hit: &super::Hit) -> Color {
        ERROR_COLOR
    }
}
