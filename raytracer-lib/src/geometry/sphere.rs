use std::sync::Arc;

use super::{BBox, Shape, ShapeType};
use crate::shader::{NullShader, Shader};
use crate::{prelude::*, vec3};
use approx::relative_eq;

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: Real,
    bbox: BBox,
    shader: Arc<dyn Shader>,
    name: &'static str,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Real, shader: Arc<dyn Shader>, name: &'static str) -> Self {
        Self {
            center,
            radius,
            bbox: BBox::new(
                center - vec3!(radius, radius, radius),
                center + vec3!(radius, radius, radius),
            ),
            shader,
            name,
        }
    }

    pub fn normal(&self, point: &Vec3) -> Vec3 {
        (point - self.center).normalize()
    }
}

impl Shape for Sphere {
    fn get_type(&self) -> ShapeType {
        ShapeType::Sphere
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn get_bbox(&self) -> &BBox {
        &self.bbox
    }

    fn get_centroid(&self) -> Vec3 {
        self.center
    }

    fn get_shader(&self) -> Arc<dyn Shader> {
        Arc::clone(&self.shader)
    }

    fn closest_hit<'hit>(&'hit self, hit: &mut crate::shader::Hit<'hit>) -> bool {
        let center_to_origin = hit.ray.origin - self.center; // vector from center of sphere to ray origin
        let d = hit.ray.direction;
        let discriminant = center_to_origin.dot(&d).powi(2)
            - d.dot(&d) * (center_to_origin.dot(&center_to_origin) - self.radius.powi(2));

        // if discriminant < 0 then there is no intersection
        if discriminant < 0.0 {
            return false;
        }

        let numerator = -center_to_origin.dot(&d);
        let denominator = d.dot(&d);

        let t1 = (numerator - discriminant.sqrt()) / denominator;
        let t2 = (numerator + discriminant.sqrt()) / denominator;
        let valid_t_range = hit.t_min..hit.t_max;

        if valid_t_range.contains(&t1) && valid_t_range.contains(&t2) {
            hit.t = t1.min(t2);
            hit.normal = self.normal(&hit.hit_point());
            hit.shape = Some(self);
            return true;
        }

        if valid_t_range.contains(&t1) {
            hit.t = t1;
            hit.normal = self.normal(&hit.hit_point());
            hit.shape = Some(self);
            return true;
        }

        if valid_t_range.contains(&t2) {
            hit.t = t2;
            hit.normal = self.normal(&hit.hit_point());
            hit.shape = Some(self);
            return true;
        }

        false
    }
}
