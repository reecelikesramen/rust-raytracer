use super::*;

use std::sync::Arc;

use na::Unit;

use crate::{shader::Shader, V3};

use super::{bbox::BBox, Shape};

#[derive(Debug)]
pub struct Triangle {
    a: P3,
    b: P3,
    c: P3,
    normal: V3,
    bbox: BBox,
    shader: Arc<dyn Shader>,
    material: Arc<dyn Material>,
    name: &'static str,
}

impl Triangle {
    pub fn new(
        a: P3,
        b: P3,
        c: P3,
        shader: Arc<dyn Shader>,
        material: Arc<dyn Material>,
        name: &'static str,
    ) -> Self {
        let normal = (b - a).cross(&(c - a)).normalize();
        let min = P3::new(
            a.x.min(b.x).min(c.x),
            a.y.min(b.y).min(c.y),
            a.z.min(b.z).min(c.z),
        );
        let max = P3::new(
            a.x.max(b.x).max(c.x),
            a.y.max(b.y).max(c.y),
            a.z.max(b.z).max(c.z),
        );
        Self {
            a,
            b,
            c,
            normal,
            bbox: BBox::new(min, max),
            shader,
            material,
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

    fn get_centroid(&self) -> P3 {
        P3::from((self.a.coords + self.b.coords + self.c.coords) / 3.0)
    }

    fn get_shader(&self) -> Arc<dyn Shader> {
        self.shader.clone()
    }

    fn get_material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }

    fn closest_hit<'hit>(&'hit self, hit: &mut crate::shader::Hit<'hit>) -> bool {
        use na::Matrix3;

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
        hit.set_normal(Unit::new_unchecked(self.normal));
        hit.shape = Some(self);

        true
    }
}
