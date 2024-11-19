use crate::{color, light::Light, prelude::*};

use super::Shader;

#[derive(Debug)]
pub struct LambertianShader {
    diffuse: Color,
}

impl LambertianShader {
    pub fn new(diffuse: Color) -> Self {
        Self { diffuse }
    }
}

impl Shader for LambertianShader {
    fn apply(&self, hit: &super::Hit) -> Color {
        let mut color = color!(0.0, 0.0, 0.0);
        for (light, surface_to_light) in hit
            .scene
            .lights
            .iter()
            .filter_map(|light| {
                light
                    .illuminates(hit)
                    .map(|surface_to_light| (light.as_ref(), surface_to_light))
            })
            .collect::<Vec<(&dyn Light, V3)>>()
        {
            let cos_incidence = hit.normal.dot(&surface_to_light.normalize());

            color +=
                self.diffuse.component_mul(&light.get_intensity()) * cos_incidence.max(0.0) as f32;
        }
        color
    }
}
