use std::sync::Arc;

use crate::{prelude::*, shader::Shader, vec3};

use super::{bbox::BBox, Shape};

#[derive(Debug)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    normal: Vec3,
    bbox: BBox,
    shader: Arc<dyn Shader>,
    name: &'static str,
}

impl Triangle {
    fn new(a: Vec3, b: Vec3, c: Vec3, shader: Arc<dyn Shader>, name: &'static str) -> Self {
        let normal = (b - a).cross(&(c - a)).normalize();
        let min = vec3!(
            a.x.min(b.x).min(c.x),
            a.y.min(b.y).min(c.y),
            a.z.min(b.z).min(c.z)
        );
        let max = vec3!(
            a.x.max(b.x).max(c.x),
            a.y.max(b.y).max(c.y),
            a.z.max(b.z).max(c.z)
        );
        Self {
            a,
            b,
            c,
            normal,
            bbox: BBox::new(min, max),
            shader,
            name,
        }
    }
}

impl Shape for Triangle {
    fn get_type(&self) -> super::ShapeType {
        super::ShapeType::Triangle
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn get_bbox(&self) -> &BBox {
        &self.bbox
    }

    fn get_centroid(&self) -> Vec3 {
        self.bbox.centroid
    }

    fn get_shader(&self) -> Arc<dyn Shader> {
        Arc::clone(&self.shader)
    }

    fn closest_hit<'hit>(&'hit self, hit: &mut crate::shader::Hit<'hit>) -> bool {
        use nalgebra::Matrix3;
        
        // Create the matrices for Cramer's rule
        let ab = self.a - self.b;
        let ac = self.a - self.c;
        let ao = self.a - hit.ray.origin;
        let d = hit.ray.direction;

        // Matrix A is common denominator for all calculations
        let matrix_a = Matrix3::from_columns(&[ab, ac, d]);
        let det_a = matrix_a.determinant();

        // Early exit if determinant is too close to zero (parallel to triangle)
        if det_a.abs() < Real::EPSILON {
            return false;
        }

        // Matrix for t calculation
        let matrix_t = Matrix3::from_columns(&[ab, ac, ao]);
        let det_t = matrix_t.determinant();
        let intersect_t = det_t / det_a;

        // Check if intersection is within valid range
        if intersect_t < hit.t_min || intersect_t > hit.t {
            return false;
        }

        // Matrix for gamma calculation (first barycentric coordinate)
        let matrix_gamma = Matrix3::from_columns(&[ab, ao, d]);
        let det_gamma = matrix_gamma.determinant();
        let gamma = det_gamma / det_a;

        if gamma < 0.0 || gamma > 1.0 {
            return false;
        }

        // Matrix for beta calculation (second barycentric coordinate)
        let matrix_beta = Matrix3::from_columns(&[ao, ac, d]);
        let det_beta = matrix_beta.determinant();
        let beta = det_beta / det_a;

        if beta < 0.0 || beta > 1.0 - gamma {
            return false;
        }

        // We have a valid hit, update the hit record
        hit.t = intersect_t;
        hit.normal = self.normal;
        hit.shape = Some(self);

        true
    }
}
