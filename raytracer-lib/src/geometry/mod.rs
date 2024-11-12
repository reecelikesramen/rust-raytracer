use crate::prelude::*;

mod bbox;
mod sphere;

pub use self::bbox::BBox;
pub use self::sphere::Sphere;

pub enum ShapeType {
    Sphere,
    Box,
    Triangle,
    Plane,
    Mesh,
}

pub trait Shape<'a> {
    fn get_type(&self) -> ShapeType;
    fn get_name(&self) -> &str;
    fn get_bbox(&self) -> &bbox::BBox;
    fn get_centroid(&self) -> Vec3;
    fn get_shader(&self) -> &'a dyn crate::shader::Shader;
    fn closest_hit<'hit>(&'hit self, ray: &crate::math::Ray, hit: &mut crate::shader::Hit<'hit>) -> bool;
}
