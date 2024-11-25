use crate::{hit_record::HitRecord, prelude::*};

mod ambient;
mod point;

pub use ambient::AmbientLight;
pub use point::PointLight;

pub trait Light: std::fmt::Debug + Sync {
    fn get_intensity(&self) -> Color;
    fn get_position(&self) -> P3;
    fn illuminates(&self, hit: &HitRecord) -> Option<V3>;
}
