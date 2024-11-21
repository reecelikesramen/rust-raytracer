use crate::math::{random_unit_v3, reflect};

use super::*;

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: Real,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: Real) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, hit_record: &Hit) -> Option<(Ray, Color)> {
        let reflected = reflect(&hit_record.ray.direction, &hit_record.normal).normalize()
            + self.fuzz * random_unit_v3(&mut rand::thread_rng());
        if reflected.dot(&hit_record.normal) > 0.0 {
            Some((
                Ray {
                    direction: reflected,
                    origin: hit_record.hit_point(),
                },
                self.albedo,
            ))
        } else {
            None
        }
    }
}
