use crate::{prelude::*, vec3};

use super::Light;

#[derive(Debug)]
pub struct AmbientLight {
    pub intensity: Color,
}

impl AmbientLight {
    pub fn new(intensity: Color) -> Self {
        Self { intensity }
    }
}

impl Light for AmbientLight {
    fn get_intensity(&self) -> Color {
        self.intensity
    }

    fn get_position(&self) -> crate::Vec3 {
        vec3!(0.0, 0.0, 0.0)
    }

    fn illuminates(&self, hit: &crate::shader::Hit) -> Option<Vec3> {
        Some(hit.normal)
    }
}
