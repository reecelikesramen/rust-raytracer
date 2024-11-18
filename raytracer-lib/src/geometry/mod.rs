use crate::prelude::*;

mod bbox;
mod bvh;
mod cuboid;
mod mesh;
mod sphere;
mod triangle;

pub use self::bbox::BBox;
pub use self::bvh::BVH;
pub use self::cuboid::Cuboid;
pub use self::mesh::Mesh;
pub use self::sphere::Sphere;
pub use self::triangle::Triangle;

pub enum ShapeType {
    Sphere,
    Box,
    Triangle,
    Plane,
    Mesh,
}

pub trait Shape: Send + Sync + std::fmt::Debug {
    fn get_type(&self) -> ShapeType;
    fn get_name(&self) -> &str;
    fn get_bbox(&self) -> &bbox::BBox;
    fn get_centroid(&self) -> Vec3;
    fn get_shader(&self) -> std::sync::Arc<dyn crate::shader::Shader>;
    fn closest_hit<'hit>(&'hit self, hit: &mut crate::shader::Hit<'hit>) -> bool;
}
