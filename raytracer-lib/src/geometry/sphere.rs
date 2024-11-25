use super::*;
use na::Unit;
use std::sync::Arc;

#[derive(Debug)]
pub struct Sphere {
    center: P3,
    radius: Real,
    bbox: BBox,
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
            material,
            name,
        }
    }

    #[inline(always)]
    pub fn normal(&self, point: &P3) -> Unit<V3> {
        Unit::new_normalize(point - self.center)
    }

    pub fn uv(&self, point: &P3) -> (Real, Real) {
        let local = (point - self.center) / self.radius;
        let theta = local.y.acos();
        let phi = (-local.z).atan2(local.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Shape for Sphere {
    fn get_bbox(&self) -> &BBox {
        &self.bbox
    }

    fn get_centroid(&self) -> P3 {
        self.center
    }

    fn closest_hit<'hit>(&'hit self, hit_record: &mut HitRecord<'hit>) -> bool {
        let center_to_origin = hit_record.ray.origin - self.center; // vector from center of sphere to ray origin
        let d = hit_record.ray.direction;
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
        let valid_t_range = hit_record.t_min..hit_record.t;

        let t = if valid_t_range.contains(&t1) && valid_t_range.contains(&t2) {
            t1.min(t2)
        } else if valid_t_range.contains(&t1) {
            t1
        } else if valid_t_range.contains(&t2) {
            t2
        } else {
            // no intersection
            return false;
        };

        hit_record.t = t;
        let hit_point = hit_record.point();
        hit_record.set_hit_data(
            self.normal(&hit_point),
            self.uv(&hit_point),
            self.material.clone(),
        );
        true
    }
}
