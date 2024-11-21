use crate::{
    material::Material,
    prelude::*,
    shader::{Hit, Shader},
};

use std::{fmt::Debug, sync::Arc};

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

pub trait Shape: Send + Sync + Debug {
    fn get_type(&self) -> ShapeType;
    fn get_name(&self) -> &str;
    fn get_bbox(&self) -> &BBox;
    fn get_centroid(&self) -> P3;
    fn get_shader(&self) -> Arc<dyn Shader>;
    fn get_material(&self) -> Arc<dyn Material>;
    fn closest_hit<'hit>(&'hit self, hit: &mut Hit<'hit>) -> bool;
}
