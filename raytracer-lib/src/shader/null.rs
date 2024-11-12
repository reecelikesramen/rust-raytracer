use crate::prelude::*;
use color;

pub struct NullShader;

static ERROR_COLOR: Color = color!(0.831, 0.0, 0.0);

impl super::Shader for NullShader {
	fn get_name(&self) -> &str { "null shader" }

	fn ambient(&self) -> super::Color { ERROR_COLOR }

	fn apply(&self, hit_struct: &super::Hit) -> Color { ERROR_COLOR }
}