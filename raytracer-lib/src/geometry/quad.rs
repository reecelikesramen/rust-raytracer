use na::Unit;

use crate::material::DEFAULT_MATERIAL;

use super::*;

#[derive(Debug)]
pub struct Quad {
    q: P3,
    u: V3,
    v: V3,
    bbox: BBox,
    material: Arc<dyn Material>,
    name: &'static str,
}

impl Quad {
    pub fn new(q: P3, u: V3, v: V3, material: Arc<dyn Material>, name: &'static str) -> Self {
        Self {
            q,
            u,
            v,
            bbox: BBox::new(q, q + u + v),
            material,
            name,
        }
    }
}

impl Shape for Quad {
    fn get_bbox(&self) -> &BBox {
        &self.bbox
    }

    fn get_centroid(&self) -> P3 {
        self.bbox.centroid
    }

    fn closest_hit<'hit>(&'hit self, hit_record: &mut HitRecord<'hit>) -> bool {
        use na::Matrix3;

        // Create the matrices for Cramer's rule
        let ab = -self.u;
        let ac = -self.v;
        let ao = self.q - hit_record.ray.origin;
        let d = hit_record.ray.direction;

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

        if beta < 0.0 || beta > 1.0 {
            return false;
        }

        let normal = Unit::new_normalize(self.u.cross(&self.v));

        // We have a valid hit, update the hit record
        hit_record.t = intersect_t;
        hit_record.set_hit_data(normal, (beta, gamma), self.material.clone());

        true
    }
}
