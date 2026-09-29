use crate::material::DEFAULT_MATERIAL;

use super::*;
use na::Unit;
use std::sync::Arc;

#[derive(Debug)]
pub struct Triangle {
    a: P3,
    b: P3,
    c: P3,
    normal_a: V3,
    normal_b: V3,
    normal_c: V3,
    bbox: BBox,
    material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(a: P3, b: P3, c: P3, material: Arc<dyn Material>) -> Self {
        let normal = (b - a).cross(&(c - a)).normalize();
        Self {
            a,
            b,
            c,
            normal_a: normal,
            normal_b: normal,
            normal_c: normal,
            bbox: BBox::from_points(&[a, b, c]),
            material,
        }
    }

    pub fn from_mesh(a: P3, b: P3, c: P3, normal_a: V3, normal_b: V3, normal_c: V3) -> Self {
        Self {
            a,
            b,
            c,
            normal_a,
            normal_b,
            normal_c,
            bbox: BBox::from_points(&[a, b, c]),
            material: DEFAULT_MATERIAL.clone(),
        }
    }
}

impl Shape for Triangle {
    fn get_bbox(&self) -> &BBox {
        &self.bbox
    }

    fn get_centroid(&self) -> P3 {
        P3::from((self.a.coords + self.b.coords + self.c.coords) / 3.0)
    }

    fn closest_hit(&self, hit_record: &mut HitRecord) -> bool {
        use na::Matrix3;

        // Create the matrices for Cramer's rule
        let ab = self.a - self.b;
        let ac = self.a - self.c;
        let ao = self.a - hit_record.ray.origin;
        let d = hit_record.ray.direction;

        // Matrix A is common denominator for all calculations
        let matrix_a = Matrix3::from_columns(&[ab, ac, d]);
        let det_a = matrix_a.determinant();

        // Early exit if determinant is too close to zero (parallel to triangle)
        if det_a.abs() < VERY_SMALL_NUMBER {
            return false;
        }

        // Matrix for t calculation
        let matrix_t = Matrix3::from_columns(&[ab, ac, ao]);
        let det_t = matrix_t.determinant();
        let intersect_t = det_t / det_a;

        // Check if intersection is within valid range
        if intersect_t < hit_record.t_min || intersect_t > hit_record.t {
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

        let normal =
            (1.0 - beta - gamma) * self.normal_a + beta * self.normal_b + gamma * self.normal_c;

        // We have a valid hit, update the hit record
        hit_record.t = intersect_t;
        hit_record.set_hit_data(
            Unit::new_normalize(normal),
            (beta, gamma),
            self.material.clone(),
        );

        true
    }
}
