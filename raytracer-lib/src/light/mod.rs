mod point;

pub use point::PointLight;

use crate::Color;

pub trait Light: std::fmt::Debug {
    fn get_intensity(&self) -> Color;
    fn get_position(&self) -> crate::Vec3;
    fn illuminate(&self, hit: &crate::shader::Hit) -> Color;
}
