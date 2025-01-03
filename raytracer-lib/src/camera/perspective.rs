use super::*;
use crate::math::Ray;

#[derive(Debug)]
pub struct PerspectiveCamera {
    base: CameraBase,
    focal_length: Real,
}

impl PerspectiveCamera {
    pub fn new(position: P3, view_direction: &V3, aspect_ratio: Real, focal_length: Real) -> Self {
        Self {
            base: CameraBase::new(position, view_direction, aspect_ratio),
            focal_length,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, i: u32, j: u32, di: Real, dj: Real) -> Ray {
        let (u, v) = self.base.get_uv(i, j, di, dj);
        let direction =
            self.base.basis.u * u + self.base.basis.v * v - self.base.basis.w * self.focal_length;
        Ray {
            origin: self.base.basis.position,
            direction,
        }
    }

    fn camera_base(&self) -> &CameraBase {
        &self.base
    }
    fn camera_base_mut(&mut self) -> &mut CameraBase {
        &mut self.base
    }
}
