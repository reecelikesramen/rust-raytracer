use crate::math::random_unit_v3;

use super::*;

#[derive(Debug)]
pub struct Diffuse {
    albedo: Color,
}

impl Diffuse {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, hit: &Hit) -> Option<(Ray, Color)> {
        let normal = hit.normal.into_inner();
        let random = random_unit_v3(&mut rand::thread_rng());
        let sample_hemisphere = if random.dot(&normal) > 0.0 {
            random
        } else {
            -random
        };

        let scatter_direction = normal + sample_hemisphere;

        let scatter_direction = if scatter_direction.magnitude_squared() < VERY_SMALL_NUMBER {
            normal
        } else {
            scatter_direction.normalize()
        };

        Some((
            Ray {
                origin: hit.hit_point(),
                direction: scatter_direction,
            },
            self.albedo,
        ))
    }
}
