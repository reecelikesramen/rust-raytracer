mod dielectric;
mod diffuse;
mod lambertian;
mod metal;

use std::fmt::Debug;

use crate::math::Ray;
use crate::prelude::*;
use crate::shader::Hit;

pub use dielectric::Dielectric;
pub use diffuse::Diffuse;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, hit_record: &Hit) -> Option<(Ray, Color)>;
}
