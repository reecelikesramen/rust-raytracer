
use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Ray {
    pub origin: P3,
    pub direction: V3,
}

impl Ray {
    pub fn point_at(&self, t: Real) -> P3 {
        self.origin + self.direction * t
    }

    pub fn atob(a: P3, b: P3) -> Self {
        Self {
            origin: a,
            direction: b - a,
        }
    }
}
