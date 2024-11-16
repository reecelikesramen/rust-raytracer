use crate::{
    math::Ray,
    prelude::*,
    scene::{self, Scene},
};

/// <'hit> lifetimes lives as long as a single pixel render takes.
pub struct Hit<'hit> {
    pub t: Real,
    pub t_min: Real,
    pub depth: u16,
    pub ray: crate::math::Ray,
    pub normal: Vec3,
    pub shape: Option<&'hit dyn crate::geometry::Shape>,
    pub scene: &'hit Scene,
}

impl<'hit> Hit<'hit> {
    pub fn new(ray: Ray, scene: &'hit Scene) -> Self {
        Self {
            t: INFINITY,
            t_min: 1.0,
            depth: 0,
            ray,
            normal: Vec3::default(),
            shape: None,
            scene,
        }
    }

    pub fn to_light(to_light: Ray, scene: &'hit Scene) -> Self {
        Self {
            t: 1.0,
            t_min: VERY_SMALL_NUMBER,
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
