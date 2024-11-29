use na::Unit;

use super::*;

#[derive(Debug)]
pub struct Cuboid {
    bbox: BBox,
    material: Arc<dyn Material>,
}

impl Cuboid {
    pub fn new(min: P3, max: P3, material: Arc<dyn Material>) -> Self {
        Self {
            bbox: BBox::new(min, max),
            material,
        }
    }

    fn normal(&self, hit_point: &P3) -> Unit<V3> {
        let point_to_center = hit_point - self.bbox.centroid;
        let norm_dist_along_axes = point_to_center.component_div(&self.bbox.extent).abs();

        let dx = norm_dist_along_axes.x;
        let dy = norm_dist_along_axes.y;
        let dz = norm_dist_along_axes.z;

        Unit::new_unchecked(if dx > dy && dx > dz {
            V3::new(if point_to_center.x > 0.0 { 1.0 } else { -1.0 }, 0.0, 0.0)
        } else if dy > dz {
            V3::new(0.0, if point_to_center.y > 0.0 { 1.0 } else { -1.0 }, 0.0)
        } else {
            V3::new(0.0, 0.0, if point_to_center.z > 0.0 { 1.0 } else { -1.0 })
        })
    }
}

impl Shape for Cuboid {
    fn get_bbox(&self) -> &super::BBox {
        &self.bbox
    }

    fn get_centroid(&self) -> P3 {
        self.bbox.centroid
    }

    fn closest_hit(&self, hit_record: &mut HitRecord) -> bool {
        if let Some(t) = self
            .bbox
            .hit(&hit_record.ray, hit_record.t_min, hit_record.t)
        {
            hit_record.t = t;

            let normal = self.normal(&hit_record.point());
            hit_record.set_hit_data(normal, (0., 0.), self.material.clone());

            return true;
        }

        false
    }
}
