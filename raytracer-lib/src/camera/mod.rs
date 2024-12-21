use crate::math::{CoordinateSystem, Ray};
use crate::prelude::*;
use Real;

mod orthographic;
mod perspective;

pub use self::orthographic::OrthographicCamera;
pub use self::perspective::PerspectiveCamera;

pub trait Camera: std::fmt::Debug + Send + Sync {
    fn generate_ray(&self, i: u32, j: u32, di: Real, dj: Real, pixels_x: u32, pixels_y: u32)
        -> Ray;

    // Required methods to access the shared base
    fn camera_base(&self) -> &CameraBase;
    fn camera_base_mut(&mut self) -> &mut CameraBase;
}

#[derive(Debug)]
pub struct CameraBase {
    pub basis: CoordinateSystem,
    left: Real,
    right: Real,
    top: Real,
    bottom: Real,
}

impl CameraBase {
    pub fn new(
        position: P3,
        view_direction: &V3,
        image_plane_width: Real,
        image_plane_height: Real,
    ) -> Self {
        Self {
            basis: CoordinateSystem::new(position, view_direction),
            left: -image_plane_width / 2.0,
            right: image_plane_width / 2.0,
            top: image_plane_height / 2.0,
            bottom: -image_plane_height / 2.0,
        }
    }

    pub fn get_uv(
        &self,
        i: u32,
        j: u32,
        di: Real,
        dj: Real,
        pixels_x: u32,
        pixels_y: u32,
    ) -> (Real, Real) {
        let u = self.left + (self.right - self.left) * (i as Real + di) / pixels_x as Real;
        let v = self.bottom + (self.top - self.bottom) * (j as Real + dj) / pixels_y as Real;
        (u, v)
    }
}
