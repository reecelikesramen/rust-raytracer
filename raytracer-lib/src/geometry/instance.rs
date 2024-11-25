use super::*;

use na::{Matrix4, Rotation3, Scale3, Translation3, Unit};

#[derive(Debug)]
pub struct Instance {
    shape: Arc<dyn Shape>,
    inv_transform: Matrix4<Real>,
    normal_matrix: Matrix4<Real>,
    bbox: BBox,
    shader: Arc<dyn Shader>,
    material: Arc<dyn Material>,
    name: &'static str,
}

impl Instance {
    pub fn new(
        shape: Arc<dyn Shape>,
        translation: Translation3<Real>,
        rotation: Rotation3<Real>,
        scale: Scale3<Real>,
        shader: Arc<dyn Shader>,
        material: Arc<dyn Material>,
        name: &'static str,
    ) -> Self {
        let transform =
            translation.to_homogeneous() * rotation.to_homogeneous() * scale.to_homogeneous();
        let inv_rotate = rotation.inverse().to_homogeneous();
        let inv_scale = scale
            .try_inverse()
            .expect(&format!(
                "The scaling applied to {} is not invertible",
                name
            ))
            .to_homogeneous();
        let inv_transform = inv_scale * inv_rotate * translation.inverse().to_homogeneous();
        let normal_matrix = (inv_scale * inv_rotate).transpose();

        let bbox = shape.get_bbox().transform(&transform);
        Self {
            shape,
            inv_transform,
            normal_matrix,
            bbox,
            shader,
            material,
            name,
        }
    }
}

impl Shape for Instance {
    fn get_bbox(&self) -> &BBox {
        &self.bbox
    }

    fn get_centroid(&self) -> P3 {
        self.bbox.centroid
    }

    fn closest_hit<'hit>(&'hit self, hit_record: &mut HitRecord<'hit>) -> bool {
        let og_ray = hit_record.ray;
        let transformed_ray = crate::math::Ray {
            origin: self.inv_transform.transform_point(&og_ray.origin),
            direction: self.inv_transform.transform_vector(&og_ray.direction),
        };
        hit_record.ray = transformed_ray;

        let did_hit = self.shape.closest_hit(hit_record);

        // Reset the ray to the non-transformed ray
        hit_record.ray = og_ray;

        // Return false if no intersection
        if !did_hit {
            return false;
        }

        // Take the hit data and transform it to world space
        let hit_data = hit_record
            .hit_data
            .take()
            .expect("Hit record should have hit data");
        let normal = Unit::new_normalize(self.normal_matrix.transform_vector(&hit_data.normal));

        // Reset the hit data
        hit_record.set_hit_data(normal, hit_data.uv, self.material.clone());

        true
    }
}
