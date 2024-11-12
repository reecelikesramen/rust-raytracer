use crate::prelude::*;
use crate::math::{CoordinateSystem, Ray};
use Real;

mod perspective;
mod orthographic;

pub use self::perspective::PerspectiveCamera;
pub use self::orthographic::OrthographicCamera;

pub trait Camera {
	fn generate_ray(&self, i: u32, j: u32, di: Real, dj: Real) -> Ray;

	// Default implementation that all cameras can use
	fn set_image_pixels(&mut self, pixels_x: u32, pixels_y: u32) {
		self.camera_base_mut().pixels_x = pixels_x;
		self.camera_base_mut().pixels_y = pixels_y;
}

	// Required methods to access the shared base
	fn camera_base(&self) -> &CameraBase;
	fn camera_base_mut(&mut self) -> &mut CameraBase;
}

pub struct CameraBase {
	pub basis: CoordinateSystem,
	pub pixels_x: u32,
	pub pixels_y: u32,
	left: Real,
	right: Real,
	top: Real,
	bottom: Real
}

impl CameraBase {
	pub fn new(position: Vec3, view_direction: &Vec3, aspect_ratio: Real) -> Self {
		let image_plane_width = 0.5;
		let image_plane_height = image_plane_width / aspect_ratio;

		Self {
			basis: CoordinateSystem::new(position, view_direction),
			pixels_x: 0,
			pixels_y: 0,
			left: -image_plane_width / 2.0,
			right: image_plane_width / 2.0,
			top: image_plane_height / 2.0,
			bottom: -image_plane_height / 2.0,
		}
	}

	pub fn get_uv(&self, i: u32, j: u32, di: Real, dj: Real) -> (Real, Real) {
		let u = self.left + (self.right - self.left) * (i as Real + di) / self.pixels_x as Real;
		let v = self.bottom + (self.top - self.bottom) * (j as Real + dj) / self.pixels_y as Real;
		(u, v)
	}

	}
