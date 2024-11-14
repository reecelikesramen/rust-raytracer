use std::sync::Arc;

use super::{BBox, Shape};
use crate::{prelude::*, shader::Shader, vec3};

#[derive(Debug)]
pub struct Cuboid {
    min: Vec3,
    max: Vec3,
    bbox: BBox,
    shader: Arc<dyn Shader>,
    name: &'static str,
}

impl Cuboid {
    pub fn new(min: Vec3, max: Vec3, shader: Arc<dyn Shader>, name: &'static str) -> Self {
        Self {
            min,
            max,
            bbox: BBox::new(min, max),
            shader,
            name,
        }
    }
}

impl Shape for Cuboid {
    fn get_type(&self) -> super::ShapeType {
        super::ShapeType::Box
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn get_bbox(&self) -> &super::BBox {
        &self.bbox
    }

    fn get_centroid(&self) -> Vec3 {
        self.bbox.centroid
    }

    fn get_shader(&self) -> Arc<dyn Shader> {
        Arc::clone(&self.shader)
    }

    fn closest_hit<'hit>(&'hit self, hit: &mut crate::shader::Hit<'hit>) -> bool {
        if let Some(t) = self.bbox.hit(&hit.ray, hit.t_min, hit.t) {
            hit.t = t;
            hit.shape = Some(self);

            let point_to_center = hit.hit_point() - self.bbox.centroid;
            let norm_dist_along_axes = point_to_center.component_div(&self.bbox.extent);

            let [dx, dy, dz] = norm_dist_along_axes.into();

            if dx > dy && dx > dz {
                hit.normal = vec3!(if point_to_center.x > 0.0 { 1.0 } else { -1.0 }, 0.0, 0.0);
            } else if dy > dz {
                hit.normal = vec3!(0.0, if point_to_center.y > 0.0 { 1.0 } else { -1.0 }, 0.0);
            } else {
                hit.normal = vec3!(0.0, 0.0, if point_to_center.z > 0.0 { 1.0 } else { -1.0 });
            }

            return true;
        }

        false
    }
}
