use super::*;
use tobj::Model;

#[derive(Debug)]
pub struct Mesh {
    bvh: BVH,
    bbox: BBox,
    material: Arc<dyn Material>,
}

impl Mesh {
    pub fn new(model: Model, material: Arc<dyn Material>) -> Self {
        let positions = model
            .mesh
            .positions
            .chunks(3)
            .map(|p| P3::new(p[0] as f64, p[1] as f64, p[2] as f64))
            .collect::<Vec<P3>>();
        let triangles = model
            .mesh
            .indices
            .chunks(3)
            .map(|i| {
                Arc::new(Triangle::new(
                    positions[i[0] as usize],
                    positions[i[1] as usize],
                    positions[i[2] as usize],
                    material.clone(),
                )) as Arc<dyn Shape>
            })
            .collect::<Vec<Arc<dyn Shape>>>();
        let bvh = BVH::new(triangles);
        let bbox = bvh.get_bbox().clone();
        Self {
            bvh,
            bbox,
            material,
        }
    }
}

impl Shape for Mesh {
    fn get_bbox(&self) -> &BBox {
        &self.bbox
    }

    fn get_centroid(&self) -> P3 {
        self.bbox.centroid
    }

    fn closest_hit(&self, hit_record: &mut HitRecord) -> bool {
        let did_hit = self.bvh.closest_hit(hit_record);

        // Return false if no intersection
        if !did_hit {
            return false;
        }

        // Set the material
        hit_record
            .hit_data
            .as_mut()
            .expect("Hit record should have hit data")
            .material = self.material.clone();

        true
    }
}
