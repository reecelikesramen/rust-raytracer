use crate::prelude::*;

mod bbox;
mod bvh;
mod cuboid;
mod instance;
mod mesh;
mod sphere;
mod triangle;

pub use bbox::BBox;
pub use bvh::BVH;
pub use cuboid::Cuboid;
pub use instance::Instance;
pub use mesh::Mesh;
pub use sphere::Sphere;
pub use triangle::Triangle;

pub enum ShapeType {
    Sphere,
    Box,
    Triangle,
    Mesh,
    Instance,
    Plane,
}

pub trait Shape: Send + Sync + std::fmt::Debug {
    fn get_type(&self) -> ShapeType;
    fn get_name(&self) -> &str;
    fn get_bbox(&self) -> &bbox::BBox;
    fn get_centroid(&self) -> P3;
    fn get_shader(&self) -> std::sync::Arc<dyn crate::shader::Shader>;
    fn closest_hit<'hit>(&'hit self, hit: &mut crate::shader::Hit<'hit>) -> bool;
}
