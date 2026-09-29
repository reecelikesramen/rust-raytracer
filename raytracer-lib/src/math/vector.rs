use na::Unit;
use rand::Rng;

use crate::prelude::*;

pub fn random_unit_v3(rng: &mut rand::rngs::ThreadRng) -> V3 {
    loop {
        let random = V3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        let magnitude_squared = random.magnitude_squared();

        if magnitude_squared <= 1.0 && magnitude_squared > VERY_SMALL_NUMBER {
            return random.normalize();
        }
    }
}

pub fn reflect(vec: &V3, normal: &Unit<V3>) -> V3 {
    let normal = normal.into_inner();

    vec - 2.0 * vec.dot(&normal) * normal
}

pub fn refract(vec: &Unit<V3>, normal: &Unit<V3>, relative_refrative_index: Real) -> V3 {
    let (vec, normal) = (vec.into_inner(), normal.into_inner());
    let cos_theta = -vec.dot(&normal).min(1.0);
    let ray_perpendicular = relative_refrative_index * (vec + cos_theta * normal);
    let ray_parallel = -(1.0 - ray_perpendicular.magnitude_squared()).sqrt() * normal;

    ray_perpendicular + ray_parallel
}
