use na::Unit;

use crate::math::refract;

use super::*;

#[derive(Debug)]
pub struct Dielectric {
    refractive_index: Real,
}

impl Dielectric {
    pub fn new(refractive_index: Real) -> Self {
        Self { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit_record: &Hit) -> Option<(Ray, Color)> {
        let refractive_index = if hit_record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let refracted = refract(
            &Unit::new_normalize(hit_record.ray.direction),
            &hit_record.normal,
            refractive_index,
        );

        Some((
            Ray {
                origin: hit_record.hit_point(),
                direction: refracted,
            },
            Color::new(0.99, 0.99, 0.99),
        ))
    }
}
