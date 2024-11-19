use crate::prelude::*;

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

    fn get_position(&self) -> P3 {
        P3::default()
    }

    fn illuminates(&self, hit: &crate::shader::Hit) -> Option<V3> {
        Some(hit.normal.into_inner())
    }
}
