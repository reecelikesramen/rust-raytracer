use na::Unit;
use rand::Rng;

use crate::math::{reflect, refract};

use super::*;

#[derive(Debug)]
pub struct Dielectric {
    attenuation: Color,
    refractive_index: Real,
}

impl Dielectric {
    pub fn new(attenuation: Color, refractive_index: Real) -> Self {
        Self {
            attenuation,
            refractive_index,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit_record: &Hit) -> Option<(Ray, Color)> {
        let refractive_index = if hit_record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = Unit::new_normalize(hit_record.ray.direction);
        let cosine = -unit_direction.dot(&hit_record.normal).min(1.0);
        let sine = (1.0 - cosine * cosine).sqrt();

        let random: Real = rand::thread_rng().gen();
        let direction =
            if refractive_index * sine > 1.0 || reflectance(cosine, refractive_index) > random {
                // cannot refract, must reflect
                reflect(&unit_direction, &hit_record.normal)
            } else {
                refract(&unit_direction, &hit_record.normal, refractive_index)
            };

        Some((
            Ray {
                origin: hit_record.hit_point(),
                direction,
            },
            self.attenuation,
        ))
    }
}

fn reflectance(cosine: Real, refractive_index: Real) -> Real {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
