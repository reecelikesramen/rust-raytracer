use crate::math::{random_unit_v3, reflect};

use super::*;

#[derive(Debug)]
pub struct Metal {
    albedo: Arc<dyn Texture>,
    fuzz: Real,
}

impl Metal {
    pub fn new(albedo: Arc<dyn Texture>, fuzz: Real) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, hit_record: &HitRecord, hit_data: &HitData) -> Option<(Ray, Color)> {
        let reflected = reflect(&hit_record.ray.direction, &hit_data.normal).normalize()
            + self.fuzz * random_unit_v3(&mut rand::thread_rng());
        if reflected.dot(&hit_data.normal) > 0.0 {
            let hit_point = hit_record.point();
            Some((
                Ray {
                    origin: hit_point,
                    direction: reflected,
                },
                self.albedo.color(hit_data.uv, &hit_point),
            ))
        } else {
            None
        }
    }
}
