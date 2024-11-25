use na::Unit;
use rand::Rng;

use crate::{color, math::Ray, prelude::*};

use super::{HitRecord, Shader};

#[derive(Debug)]
pub struct DiffuseShader {
    diffuse: Color,
    samples: u32,
}

impl DiffuseShader {
    pub fn new(diffuse: Color, samples: u32) -> Self {
        Self { diffuse, samples }
    }
}

impl Shader for DiffuseShader {
    fn apply(&self, hit: &HitRecord) -> Color {
        panic!("reworking");
        // if hit.depth >= hit.scene.recursion_depth {
        //     return hit.scene.background_color;
        // }

        // let mut color = color!(0.0, 0.0, 0.0);

        // // Monte Carlo integration for multi-sampling
        // let mut rng = rand::thread_rng();
        // let mut indirect_color = color!(0.0, 0.0, 0.0);
        // for _ in 0..self.samples {
        //     let outgoing = sample_hemisphere(&mut rng, &hit.normal);
        //     let mut indirect_hit = hit.bounce(Ray {
        //         origin: hit.point(),
        //         direction: outgoing.into_inner(),
        //     });

        //     hit.scene.bvh.closest_hit(&mut indirect_hit);

        //     let cos_incidence = hit.normal.dot(&outgoing).max(0.0);

        //     indirect_color +=
        //         self.diffuse.component_mul(&indirect_hit.hit_color()) * cos_incidence as f32;
        // }

        // color += indirect_color / self.samples as f32;

        // color
    }
}

fn sample_hemisphere(rng: &mut rand::rngs::ThreadRng, normal: &V3) -> Unit<V3> {
    let random = V3::new(
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
    );

    let random = if random.dot(normal) > 0.0 {
        random
    } else {
        -random
    };

    Unit::new_normalize(random)
}
