use super::{BBox, ShapeType};
use crate::prelude::*;
use crate::shader::{NullShader, Shader};
use approx::relative_eq;
use vec3;

static NULL_SHADER: NullShader = NullShader {};

pub struct Sphere<'a> {
    center: Vec3,
    radius: Real,
    bbox: BBox,
    shader: &'a dyn Shader,
    name: &'a str,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: Real) -> Self {
        Self {
            center,
            radius,
            bbox: BBox::new(
                center - vec3!(radius, radius, radius),
                center + vec3!(radius, radius, radius),
            ),
            shader: &NULL_SHADER,
            name: "unnamed sphere",
        }
    }

    pub fn normal(&self, point: &Vec3) -> Vec3 {
        (point - self.center).normalize()
    }
}

impl<'a> super::Shape<'a> for Sphere<'a> {
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

    fn get_shader(&self) -> &'a dyn Shader {
        self.shader
    }

    fn closest_hit<'hit>(&'hit self, ray: &crate::math::Ray, hit: &mut crate::shader::Hit<'hit>) -> bool {
        let center_to_origin = ray.origin - self.center; // vector from center of sphere to ray origin
        let d = ray.direction;
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
            hit.normal = self.normal(&ray.point_at(hit.t));
            hit.shape = Some(self);
            return true;
        }

        if valid_t_range.contains(&t1) {
            hit.t = t1;
            hit.normal = self.normal(&ray.point_at(hit.t));
            hit.shape = Some(self);
            return true;
        }

        if valid_t_range.contains(&t2) {
            hit.t = t2;
            hit.normal = self.normal(&ray.point_at(hit.t));
            hit.shape = Some(self);
            return true;
        }

        false
    }
}
