use na::Unit;
use rand::Rng;

use crate::math::{reflect, refract};

use super::*;

#[derive(Debug)]
pub struct Dielectric {
    attenuation: Arc<dyn Texture>,
    refractive_index: Real,
}

impl Dielectric {
    pub fn new(attenuation: Arc<dyn Texture>, refractive_index: Real) -> Self {
        Self {
            attenuation,
            refractive_index,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit_record: &HitRecord, hit_data: &HitData) -> Option<(Ray, Color)> {
        let refractive_index = if hit_data.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = Unit::new_normalize(hit_record.ray.direction);
        let cosine = -unit_direction.dot(&hit_data.normal).min(1.0);
        let sine = (1.0 - cosine * cosine).sqrt();

        let random: Real = rand::thread_rng().gen();
        let direction =
            if refractive_index * sine > 1.0 || reflectance(cosine, refractive_index) > random {
                // cannot refract, must reflect
                reflect(&unit_direction, &hit_data.normal)
            } else {
                refract(&unit_direction, &hit_data.normal, refractive_index)
            };

        let hit_point = hit_record.point();
        Some((
            Ray {
                origin: hit_point,
                direction,
            },
            self.attenuation.color(hit_data.uv, &hit_point),
        ))
    }
}

fn reflectance(cosine: Real, refractive_index: Real) -> Real {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
