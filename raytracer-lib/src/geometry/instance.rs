use crate::prelude::*;
use std::sync::Arc;

use na::{Matrix4, Rotation3, Scale3, Translation3, Unit};

use crate::shader::Shader;

use super::{bbox::BBox, Real, Shape, ShapeType};

#[derive(Debug)]
pub struct Instance {
    shape: Arc<dyn Shape>,
    inv_transform: Matrix4<Real>,
    normal_matrix: Matrix4<Real>,
    bbox: BBox,
    shader: Arc<dyn Shader>,
    name: &'static str,
}

impl Instance {
    pub fn new(
        shape: Arc<dyn Shape>,
        translation: Translation3<Real>,
        rotation: Rotation3<Real>,
        scale: Scale3<Real>,
        shader: Arc<dyn Shader>,
        name: &'static str,
    ) -> Self {
        let scaling_err_msg = format!("The scaling applied to {} is not invertible", name);
        let transform =
            translation.to_homogeneous() * rotation.to_homogeneous() * scale.to_homogeneous();
        let inv_transform = translation.inverse().to_homogeneous()
            * rotation.inverse().to_homogeneous()
            * scale
                .try_inverse()
                .expect(&scaling_err_msg)
                .to_homogeneous();
        let normal_matrix = rotation.inverse().to_homogeneous()
            * scale
                .try_inverse()
                .expect(&scaling_err_msg)
                .to_homogeneous();

        let bbox = shape.get_bbox().transform(&transform);
        Self {
            shape,
            inv_transform,
            normal_matrix,
            bbox,
            shader,
            name,
        }
    }
}

impl Shape for Instance {
    fn get_type(&self) -> ShapeType {
        ShapeType::Instance
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn get_bbox(&self) -> &BBox {
        &self.bbox
    }

    fn get_centroid(&self) -> P3 {
        self.bbox.centroid
    }

    fn get_shader(&self) -> Arc<dyn Shader> {
        self.shader.clone()
    }

    fn closest_hit<'hit>(&'hit self, hit: &mut crate::shader::Hit<'hit>) -> bool {
        let og_ray = hit.ray;
        let transformed_ray = crate::math::Ray {
            origin: self.inv_transform.transform_point(&og_ray.origin),
            direction: self.inv_transform.transform_vector(&og_ray.direction),
        };
        hit.ray = transformed_ray;

        if !self.shape.closest_hit(hit) {
            return false;
        }

        // ray is in model local space
        // normal is in model local space

        let normal = self.normal_matrix.transform_vector(&hit.normal);

        hit.normal = Unit::new_normalize(normal);
        hit.ray = og_ray;
        hit.shape = Some(self);

        true
    }
}
