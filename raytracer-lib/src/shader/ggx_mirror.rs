use crate::{color, prelude::*, shader::perfect_mirror};
use rand::Rng;

use super::{Hit, Shader};

#[derive(Debug)]
pub struct GGXMirrorShader {
    roughness: Real,
    samples: u32, // Number of samples for roughness approximation
}

impl GGXMirrorShader {
    pub fn new(roughness: Real, samples: u32) -> Self {
        Self {
            roughness: roughness.clamp(0.0, 1.0),
            samples,
        }
    }

    // GGX/Trowbridge-Reitz distribution function
    fn ggx_distribution(&self, n_dot_h: Real) -> Real {
        let alpha2 = self.roughness * self.roughness;
        let nom = alpha2;
        let denom = PI * (n_dot_h * n_dot_h * (alpha2 - 1.0) + 1.0).powi(2);
        nom / denom.max(VERY_SMALL_NUMBER)
    }

    // Generate a microfacet normal using GGX distribution
    fn sample_ggx(&self, normal: &Vec3, rng: &mut impl Rng) -> Vec3 {
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

        // Multi-sample the roughness
        for _ in 0..self.samples {
            // Sample microfacet normal using GGX distribution
            let micro_normal = self.sample_ggx(&hit.normal, &mut rng);

            // Calculate reflected direction using the sampled microfacet normal
            let outgoing = incoming - micro_normal * (2.0 * incoming.dot(&micro_normal));

            let mut mirror_hit = Hit::new(
                crate::math::Ray {
                    origin: hit.hit_point(),
                    direction: outgoing.normalize(),
                },
                &hit.scene,
            );
            mirror_hit.depth = hit.depth + 1;
            mirror_hit.t_min = VERY_SMALL_NUMBER;

            // Get color for this sample
            let sample_color = if hit.scene.bvh.closest_hit(&mut mirror_hit) {
                mirror_hit.shape.unwrap().get_shader().apply(&mirror_hit)
            } else {
                hit.scene.background_color
            };

            accumulated_color += sample_color;
        }

        // Average the samples
        accumulated_color / self.samples as f32
    }
}

// Helper function to create a coordinate system from a normal
fn create_coordinate_system(normal: &Vec3) -> (Vec3, Vec3) {
    let tangent = if normal.x.abs() > 0.99 {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let bitangent = normal.cross(&tangent).normalize();
    let tangent = bitangent.cross(normal).normalize();
    (tangent, bitangent)
}
