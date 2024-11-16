use rand::Rng;

use crate::{prelude::*, shader::perfect_mirror};

use super::{Hit, Shader};

#[derive(Debug, Default)]
pub struct PerfectMirrorShader;

impl Shader for PerfectMirrorShader {
    fn apply(&self, hit: &Hit) -> Color {
        if (hit.depth >= hit.scene.recursion_depth) {
            return hit.scene.background_color;
        }

        // incoming ray is not pre-normalized when we get here
        let incoming = hit.ray.direction.normalize();
        let outgoing = hit.normal * (2.0 * -incoming.dot(&hit.normal)) + incoming;
        let mut mirror_hit = Hit::new(
            crate::math::Ray {
                origin: hit.hit_point(),
                direction: outgoing,
            },
            &hit.scene,
        );
        mirror_hit.depth = hit.depth + 1;
        mirror_hit.t_min = VERY_SMALL_NUMBER;

        if hit.scene.bvh.closest_hit(&mut mirror_hit) {
            mirror_hit.shape.unwrap().get_shader().apply(&mirror_hit)
        } else {
            hit.scene.background_color
        }
    }
}
