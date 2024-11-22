use na::Unit;

use crate::{math::Ray, prelude::*, scene::Scene};

/// <'hit> lifetimes lives as long as a single pixel render takes.
pub struct Hit<'hit> {
    pub t: Real,
    pub t_min: Real,
    pub depth: u16,
    pub ray: crate::math::Ray,
    pub normal: Unit<V3>,
    pub front_face: bool,
    pub shape: Option<&'hit dyn crate::geometry::Shape>,
    pub scene: &'hit Scene,
}

impl<'hit> Hit<'hit> {
    pub fn new(ray: Ray, scene: &'hit Scene) -> Self {
        Self {
            t: INFINITY,
            t_min: 0.001,
            depth: 0,
            ray,
            normal: Unit::new_unchecked(V3::default()),
            front_face: true,
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
            normal: Unit::new_unchecked(V3::default()),
            front_face: true,
            shape: None,
            scene,
        }
    }

    pub fn hit_point(&self) -> P3 {
        self.ray.point_at(self.t)
    }

    pub fn bounce(&self, outgoing: Ray) -> Self {
        Self {
            t: INFINITY,
            t_min: VERY_SMALL_NUMBER,
            depth: self.depth + 1,
            ray: outgoing,
            normal: Unit::new_unchecked(V3::default()),
            front_face: true,
            shape: None,
            scene: self.scene,
        }
    }

    pub fn hit_color(&self) -> Color {
        match &self.shape {
            Some(s) => s.get_shader().apply(&self),
            None => self.scene.background_color,
        }
    }

    #[inline(always)]
    pub fn set_normal(&mut self, normal: Unit<V3>) {
        self.front_face = self.ray.direction.dot(&normal) < 0.0;
        self.normal = if self.front_face { normal } else { -normal };
    }
}
