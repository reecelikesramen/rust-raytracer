use std::sync::Arc;

use tobj::Model;

use crate::{prelude::*, shader::Shader, vec3};

use super::{bvh, BBox, Shape, Triangle, BVH};

#[derive(Debug)]
pub struct Mesh {
    bvh: BVH,
    bbox: BBox,
    shader: Arc<dyn Shader>,
    name: &'static str,
}

impl Mesh {
    pub fn new(model: &Model, shader: Arc<dyn Shader>, name: &'static str) -> Self {
        let positions = model
            .mesh
            .positions
            .chunks(3)
            .map(|p| vec3!(p[0] as f64, p[1] as f64, p[2] as f64))
            .collect::<Vec<Vec3>>();
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

    fn get_centroid(&self) -> Vec3 {
        self.bbox.centroid
    }

    fn get_shader(&self) -> Arc<dyn Shader> {
        self.shader.clone()
    }

    fn closest_hit<'hit>(&'hit self, hit: &mut crate::shader::Hit<'hit>) -> bool {
        self.bvh.closest_hit(hit)
    }
}
