use crate::{color, shader::Hit, Color, Vec3};

use super::Light;

#[derive(Debug)]
pub struct PointLight {
    position: Vec3,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Vec3, intensity: Color) -> Self {
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

    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn illuminate(&self, hit: &Hit) -> Color {
        let surface_to_light: Vec3 = (self.get_position() - hit.hit_point());

        if !hit.scene.disable_shadows {
            // && hit.scene.any_hit()
            return color!(0.0, 0.0, 0.0);
        }

        let cos_incidence = hit.normal.dot(&surface_to_light.normalize());

        cos_incidence.max(0.0) as f32 * self.get_intensity()
    }
}
