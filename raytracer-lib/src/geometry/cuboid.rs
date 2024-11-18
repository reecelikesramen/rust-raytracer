use std::sync::Arc;

use super::{BBox, Shape};
use crate::{prelude::*, shader::Shader, vec3};

#[derive(Debug)]
pub struct Cuboid {
    bbox: BBox,
    shader: Arc<dyn Shader>,
    name: &'static str,
}

impl Cuboid {
    pub fn new(min: Vec3, max: Vec3, shader: Arc<dyn Shader>, name: &'static str) -> Self {
        Self {
            bbox: BBox::new(min, max),
            shader,
            name,
        }
    }

    fn normal(&self, hit_point: &Vec3) -> Vec3 {
        let point_to_center = hit_point - self.bbox.centroid;
        let norm_dist_along_axes = point_to_center.component_div(&self.bbox.extent).abs();

        let dx = norm_dist_along_axes.x;
        let dy = norm_dist_along_axes.y;
        let dz = norm_dist_along_axes.z;

        if dx > dy && dx > dz {
            vec3!(if point_to_center.x > 0.0 { 1.0 } else { -1.0 }, 0.0, 0.0)
        } else if dy > dz {
            vec3!(0.0, if point_to_center.y > 0.0 { 1.0 } else { -1.0 }, 0.0)
        } else {
            vec3!(0.0, 0.0, if point_to_center.z > 0.0 { 1.0 } else { -1.0 })
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
            hit.normal = self.normal(&hit.hit_point());
            hit.shape = Some(self);

            return true;
        }

        false
    }
}
