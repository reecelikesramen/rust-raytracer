use crate::prelude::*;

use super::{Hit, Shader};

#[derive(Debug, Default)]
pub struct PerfectMirrorShader;

impl Shader for PerfectMirrorShader {
    fn apply(&self, hit: &Hit) -> Color {
        if hit.depth >= hit.scene.recursion_depth {
            return hit.scene.background_color;
        }

        // incoming ray is not pre-normalized when we get here
        let incoming = hit.ray.direction.normalize();
        let outgoing = hit.normal.into_inner() * (2.0 * -incoming.dot(&hit.normal)) + incoming;
        let mut mirror_hit = hit.bounce(crate::math::Ray {
            origin: hit.hit_point(),
            direction: outgoing,
        });

        hit.scene.bvh.closest_hit(&mut mirror_hit);

        mirror_hit.hit_color()
    }
}
