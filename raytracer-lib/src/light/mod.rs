use crate::prelude::*;

mod ambient;
mod point;

pub use ambient::AmbientLight;
pub use point::PointLight;

pub trait Light: std::fmt::Debug {
    fn get_intensity(&self) -> Color;
    fn get_position(&self) -> crate::Vec3;
    fn illuminates(&self, hit: &crate::shader::Hit) -> Option<Vec3>;
}
