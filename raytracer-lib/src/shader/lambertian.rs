use crate::{color, constants::DEFAULT_AMBIENT_LIGHT, prelude::*};

use super::Shader;

#[derive(Debug)]
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
            ambient: ambient.unwrap_or(DEFAULT_AMBIENT_LIGHT),
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
        let mut color = color!(0.0, 0.0, 0.0);
        for light in &hit.scene.lights {
            color += self.diffuse.component_mul(&light.illuminate(hit));
        }
        color
    }
}
