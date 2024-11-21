use crate::{
    color,
    math::{random_unit_v3, Ray},
    prelude::*,
};

use super::Shader;

#[derive(Debug)]
pub struct LambertianShader {
    diffuse: Color,
    samples: u32,
}

impl LambertianShader {
    pub fn new(diffuse: Color, samples: u32) -> Self {
        Self { diffuse, samples }
    }
}

impl Shader for LambertianShader {
    fn apply(&self, hit: &super::Hit) -> Color {
        if hit.depth >= hit.scene.recursion_depth {
            return hit.scene.background_color;
        }

        let mut color = color!(0.0, 0.0, 0.0);

        // Monte Carlo integration for multi-sampling
        let mut rng = rand::thread_rng();
        for _ in 0..self.samples {
            let outgoing = (random_unit_v3(&mut rng) + hit.normal.into_inner()).normalize();
            let mut indirect_hit = hit.bounce(Ray {
                origin: hit.hit_point(),
                direction: outgoing,
            });

            hit.scene.bvh.closest_hit(&mut indirect_hit);

            let cos_incidence = hit.normal.dot(&outgoing).max(0.0);

            color += self.diffuse.component_mul(&indirect_hit.hit_color()) * cos_incidence as f32;
        }

        color / self.samples as f32
    }
}
