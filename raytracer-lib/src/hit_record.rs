use std::sync::Arc;

use na::Unit;

use crate::{material::Material, math::Ray, prelude::*};

pub struct HitData {
    pub uv: (Real, Real),
    pub normal: Unit<V3>,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

pub struct HitRecord {
    pub t: Real,
    pub t_min: Real,
    pub depth: u16,
    pub ray: Ray,
    pub hit_data: Option<HitData>,
}

impl HitRecord {
    pub fn new(ray: Ray) -> Self {
        Self {
            t: INFINITY,
            t_min: 0.001,
            depth: 0,
            ray,
            hit_data: None,
        }
    }

    pub fn bounce(&self, outgoing: Ray) -> Self {
        Self {
            t: INFINITY,
            t_min: VERY_SMALL_NUMBER,
            depth: self.depth + 1,
            ray: outgoing,
            hit_data: None,
        }
    }

    #[inline(always)]
    pub fn point(&self) -> P3 {
        self.ray.point_at(self.t)
    }

    #[inline(always)]
    pub fn set_hit_data(
        &mut self,
        normal: Unit<V3>,
        uv: (Real, Real),
        material: Arc<dyn Material>,
    ) {
        let front_face = self.ray.direction.dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        self.hit_data = Some(HitData {
            normal,
            uv,
            front_face,
            material,
        });
    }
}
