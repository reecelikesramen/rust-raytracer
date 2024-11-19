use std::sync::Arc;

use tobj::load_obj;

use crate::{prelude::*, shader::Shader};

use super::{BBox, Shape, Triangle, BVH};

#[derive(Debug)]
pub struct Mesh {
    bvh: BVH,
    bbox: BBox,
    shader: Arc<dyn Shader>,
    name: &'static str,
}

impl Mesh {
    pub fn new(model_path: String, shader: Arc<dyn Shader>, name: &'static str) -> Self {
        let (models, _) = load_obj(
            model_path,
            &tobj::LoadOptions {
                triangulate: true,
                ..Default::default()
            },
        )
        .expect("Failed to load model for mesh");

        if models.len() != 1 {
            panic!(
                "expected exactly one model, found {} for mesh {}",
                models.len(),
                name
            );
        }

        // take ownership of the model from the Vec
        let model = models.into_iter().next().unwrap();

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
                    shader.clone(),
                    name,
                )) as Arc<dyn Shape>
            })
            .collect::<Vec<Arc<dyn Shape>>>();
        let bvh = BVH::new(triangles);
        let bbox = bvh.get_bbox().clone();
        Self {
            bvh,
            bbox,
            shader,
            name,
        }
    }
}

impl Shape for Mesh {
    fn get_type(&self) -> super::ShapeType {
        super::ShapeType::Mesh
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
        self.bvh.closest_hit(hit)
    }
}
