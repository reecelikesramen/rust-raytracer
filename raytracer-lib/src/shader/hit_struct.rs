use na::Unit;

use crate::{math::Ray, prelude::*, scene::Scene};

/// <'hit> lifetimes lives as long as a single pixel render takes.
pub struct Hit<'hit> {
    pub t: Real,
    pub t_min: Real,
    pub depth: u16,
    pub ray: crate::math::Ray,
    pub normal: Unit<V3>,
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
            normal: Unit::new_unchecked(V3::default()),
            shape: None,
            scene,
        }
    }

    pub fn to_light(to_light: Ray, scene: &'hit Scene) -> Self {
        Self {
            t: 1.0,
            t_min: 0.01,
            depth: 0,
            ray: to_light,
            normal: Unit::new_unchecked(V3::default()),
            shape: None,
            scene,
        }
    }

    pub fn hit_point(&self) -> P3 {
        self.ray.point_at(self.t)
    }
}
