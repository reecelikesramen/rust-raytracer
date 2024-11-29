use crate::math::random_unit_v3;

use super::*;

#[derive(Debug)]
pub struct Diffuse {
    albedo: Arc<dyn Texture>,
}

impl Diffuse {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, hit: &HitRecord, hit_data: &HitData) -> Option<(Ray, Color)> {
        let normal = hit_data.normal.into_inner();
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

        let hit_point = hit.point();
        Some((
            Ray {
                origin: hit_point,
                direction: scatter_direction,
            },
            self.albedo.color(hit_data.uv, &hit_point),
        ))
    }
}
