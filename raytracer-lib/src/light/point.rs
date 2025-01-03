use crate::prelude::*;
use crate::{math::Ray, shader::Hit};

use super::Light;

#[derive(Debug)]
pub struct PointLight {
    position: P3,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: P3, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

impl Light for PointLight {
    fn get_intensity(&self) -> Color {
        self.intensity
    }

    fn get_position(&self) -> P3 {
        self.position
    }

    fn illuminates(&self, hit: &Hit) -> Option<V3> {
        let surface_to_light = Ray::atob(hit.hit_point(), self.get_position());
        let mut shadow_hit = Hit::to_light(surface_to_light, &hit.scene);

        // if shadows are enabled and a shape blocks the light
        if !hit.scene.disable_shadows && hit.scene.bvh.closest_hit(&mut shadow_hit) {
            return None;
        }

        Some(surface_to_light.direction)
    }
}
