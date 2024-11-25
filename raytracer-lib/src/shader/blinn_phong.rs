use crate::{color, light::Light, prelude::*};

use super::Shader;

#[derive(Debug)]
pub struct BlinnPhongShader {
    diffuse: Color,
    specular: Color,
    shininess: f32,
}

impl BlinnPhongShader {
    pub fn new(diffuse: Color, specular: Color, shininess: f32) -> Self {
        Self {
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Shader for BlinnPhongShader {
    fn apply(&self, hit: &super::HitRecord) -> Color {
        panic!("reworking");
        // let mut color = color!(0.0, 0.0, 0.0);
        // for (light, surface_to_light) in hit
        //     .scene
        //     .lights
        //     .iter()
        //     .filter_map(|light| {
        //         light
        //             .illuminates(hit)
        //             .map(|surface_to_light| (light.as_ref(), surface_to_light))
        //     })
        //     .collect::<Vec<(&dyn Light, V3)>>()
        // {
        //     let stol_normal = surface_to_light.normalize();
        //     let cos_incidence = hit.normal.dot(&stol_normal);

        //     color +=
        //         self.diffuse.component_mul(&light.get_intensity()) * cos_incidence.max(0.0) as f32;

        //     let half_vector = ((-hit.ray.direction.normalize()) + stol_normal).normalize();
        //     color += self.specular.component_mul(&light.get_intensity())
        //         * (hit.normal.dot(&half_vector).max(0.0) as f32).powf(self.shininess);
        // }
        // color
    }
}
