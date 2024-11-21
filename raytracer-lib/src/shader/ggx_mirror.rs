use crate::{color, math::create_coordinate_system, prelude::*};
use rand::Rng;

use super::{Hit, Shader};

#[derive(Debug)]
pub struct GGXMirrorShader {
    roughness: Real,
    samples: u32,
}

impl GGXMirrorShader {
    pub fn new(roughness: Real, samples: u32) -> Self {
        Self {
            roughness: roughness.clamp(0.0, 1.0),
            samples,
        }
    }

    // Generate a microfacet normal using GGX distribution
    fn sample_ggx(&self, normal: &V3, rng: &mut impl Rng) -> V3 {
        let alpha = self.roughness;

        // Sample uniformly on unit square [0,1]^2
        let u1 = rng.gen::<Real>();
        let u2 = rng.gen::<Real>();

        // GGX Distribution sampling
        let phi = 2.0 * PI * u1;
        let theta = (alpha * alpha * u2 / (1.0 - u2)).sqrt().atan();

        // Convert to cartesian coordinates
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let x = sin_theta * phi.cos();
        let y = sin_theta * phi.sin();
        let z = cos_theta;

        // Transform to world space
        let (tangent, bitangent) = create_coordinate_system(normal);
        tangent * x + bitangent * y + normal * z
    }
}

impl Shader for GGXMirrorShader {
    fn apply(&self, hit: &Hit) -> Color {
        if hit.depth >= hit.scene.recursion_depth {
            return hit.scene.background_color;
        }

        let mut rng = rand::thread_rng();
        let incoming = hit.ray.direction.normalize();
        let mut accumulated_color = color!(0.0, 0.0, 0.0);

        // Sample microfacet normal using GGX distribution
        // let micro_normal = self.sample_ggx(&hit.normal, &mut rng);

        // Calculate reflected direction using the sampled microfacet normal
        // let outgoing = incoming - micro_normal * (2.0 * incoming.dot(&micro_normal));

        // Take self.samples samples of the micro normal and calculate the outgoing vector
        let mut outgoing_samples = (0..self.samples)
            .map(|_| self.sample_ggx(&hit.normal, &mut rng))
            .map(|micro_normal| incoming - micro_normal * (2.0 * incoming.dot(&micro_normal)))
            .collect::<Vec<V3>>();

        // sort this by the outgoing vector dot product with the hit normal
        outgoing_samples.sort_by(|a, b| a.dot(&hit.normal).total_cmp(&b.dot(&hit.normal)));

        for outgoing in outgoing_samples {
            let mut mirror_hit = hit.bounce(crate::math::Ray {
                origin: hit.hit_point(),
                direction: outgoing.normalize(),
            });

            // Get color for this sample
            hit.scene.bvh.closest_hit(&mut mirror_hit);

            accumulated_color += mirror_hit.hit_color();
        }

        // Multi-sample the roughness
        // for _ in 0..self.samples {
        //     let mut mirror_hit = hit.bounce(crate::math::Ray {
        //         origin: hit.hit_point(),
        //         direction: outgoing.normalize(),
        //     });

        //     // Get color for this sample
        //     hit.scene.bvh.closest_hit(&mut mirror_hit);

        //     accumulated_color += mirror_hit.hit_color();
        // }

        // Average the samples
        accumulated_color / self.samples as f32
    }
}
