use std::sync::Arc;

use na::Unit;

use super::*;
use super::{BBox, Shape, ShapeType};
use crate::shader::Shader;

#[derive(Debug)]
pub struct Sphere {
    center: P3,
    radius: Real,
    bbox: BBox,
    shader: Arc<dyn Shader>,
    material: Arc<dyn Material>,
    name: &'static str,
}

impl Sphere {
    pub fn new(
        center: P3,
        radius: Real,
        shader: Arc<dyn Shader>,
        material: Arc<dyn Material>,
        name: &'static str,
    ) -> Self {
        Self {
            center,
            radius,
            bbox: BBox::new(
                center - V3::new(radius, radius, radius),
                center + V3::new(radius, radius, radius),
            ),
            shader,
            material,
            name,
        }
    }

    #[inline(always)]
    pub fn normal(&self, point: &P3) -> Unit<V3> {
        Unit::new_normalize(point - self.center)
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

    fn get_centroid(&self) -> P3 {
        self.center
    }

    fn get_shader(&self) -> Arc<dyn Shader> {
        self.shader.clone()
    }

    fn get_material(&self) -> Arc<dyn Material> {
        self.material.clone()
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
        let valid_t_range = hit.t_min..hit.t;

        if valid_t_range.contains(&t1) && valid_t_range.contains(&t2) {
            hit.t = t1.min(t2);
        } else if valid_t_range.contains(&t1) {
            hit.t = t1;
        } else if valid_t_range.contains(&t2) {
            hit.t = t2;
        } else {
            return false;
        }

        hit.set_normal(self.normal(&hit.hit_point()));
        hit.shape = Some(self);
        true
    }
}
