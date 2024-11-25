use crate::{hit_record::HitRecord, material::Material, prelude::*, shader::Shader};

use std::{fmt::Debug, sync::Arc};

mod bbox;
mod bvh;
mod cuboid;
mod instance;
mod mesh;
mod quad;
mod sphere;
mod triangle;

pub use bbox::BBox;
pub use bvh::BVH;
pub use cuboid::Cuboid;
pub use instance::Instance;
pub use mesh::Mesh;
pub use quad::Quad;
pub use sphere::Sphere;
pub use triangle::Triangle;

pub trait Shape: Send + Sync + Debug {
    fn get_bbox(&self) -> &BBox;
    fn get_centroid(&self) -> P3;
    fn closest_hit<'hit>(&'hit self, hit_record: &mut HitRecord<'hit>) -> bool;
}
