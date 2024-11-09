use crate::prelude::*;
use crate::math::Ray;
use super::camera::*;
use vec3;

struct OrthographicCamera {
	base: CameraBase,
}

impl OrthographicCamera {
	fn new(position: Vec3, view_direction: &Vec3, aspect_ratio: Real) -> Self {
		Self {
			base: CameraBase::new(position, view_direction, aspect_ratio),
		}
	}
}

impl Camera for OrthographicCamera {
	fn generate_ray(&self, i: u32, j: u32, di: Real, dj: Real) -> Ray {
		let (u, v) = self.base.get_uv(i, j, di, dj);
		let origin = self.base.basis.position + vec3!(u, v, 0.0);
		Ray { origin, direction: vec3!(0.0, 0.0, -1.0) }
	}

	fn set_image_pixels(&mut self, pixels_x: u32, pixels_y: u32) {
		self.base.pixels_x = pixels_x;
		self.base.pixels_y = pixels_y;
	}
}