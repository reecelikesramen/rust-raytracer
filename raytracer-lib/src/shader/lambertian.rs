use crate::{constants::DEFAULT_AMBIENT_LIGHT, prelude::*};
use color;

use super::Shader;

pub struct LambertianShader {
		name: &'static str,
    diffuse: Color,
		ambient: Color,
}

impl LambertianShader {
	pub fn new(name: &'static str, diffuse: Color, ambient: Option<Color>) -> Self {
		Self {
			name,
			diffuse,
			ambient: ambient.unwrap_or(DEFAULT_AMBIENT_LIGHT)
		}
	}
}

impl Shader for LambertianShader {
    fn get_name(&self) -> &str {
			self.name
		}

		fn ambient(&self) -> Color {
			self.ambient
		}

		fn apply(&self, hit: &super::Hit) -> Color {
				// TODO: implement shadows
				let mut color = color!(0.0, 1.0, 0.0);
				color += self.diffuse * hit.normal.dot(&hit.ray.direction.normalize()).max(0.0) as f32;
				color
		}
}
