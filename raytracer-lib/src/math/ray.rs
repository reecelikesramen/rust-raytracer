use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at(&self, t: Real) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn atob(a: Vec3, b: Vec3) -> Self {
        Self {
            origin: a,
            direction: b - a,
        }
    }
}
