use crate::{constants::VERY_SMALL_NUMBER, math::Ray, prelude::*, scene::{self, Scene}};

pub struct Hit<'hit> {
    pub t: Real,
    pub t_min: Real,
    pub t_max: Real,
    pub depth: u16,
    pub ray: crate::math::Ray,
    pub normal: Vec3,
    pub shape: Option<&'hit dyn crate::geometry::Shape>,
    pub scene: &'hit Scene,
}

impl<'hit> Hit<'hit> {
    pub fn new(scene: &'hit Scene) -> Self {
        Self {
            t: INFINITY,
            t_min: 1.0,
            t_max: INFINITY,
            depth: 0,
            ray: Ray::default(),
            normal: Vec3::default(),
            shape: None,
            scene,
        }
    }

    pub fn to_light(to_light: Ray, scene: &'hit Scene) -> Self {
        Self {
            t: 1.0,
            t_min: VERY_SMALL_NUMBER,
            t_max: 1.0,
            depth: 0,
            ray: to_light,
            normal: Vec3::default(),
            shape: None,
            scene,
        }
    }

    pub fn hit_point(&self) -> Vec3 {
        self.ray.point_at(self.t)
    }
}
