use crate::{math::Ray, prelude::*};

pub struct Hit<'a> {
	pub t: Real,
	pub t_min: Real,
	pub t_max: Real,
	pub depth: u16,
	pub ray: crate::math::Ray,
	pub normal: Vec3,
	pub shape: Option<&'a dyn crate::geometry::Shape<'a>>,
	// scene: &Scene
}

impl<'a> Hit<'a> {
	pub fn new() -> Self {
		Self {
			t: INFINITY,
			t_min: 1.0,
			t_max: INFINITY,
			depth: 0,
			ray: Ray::default(),
			normal: Vec3::default(),
			shape: None
		}
	}

	pub fn hit_point(&self) -> Vec3 {
		self.ray.point_at(self.t)
	}
}