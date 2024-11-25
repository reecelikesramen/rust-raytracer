mod dielectric;
mod diffuse;
mod lambertian;
mod metal;

use std::fmt::Debug;
use std::sync::Arc;

use crate::hit_record::{HitData, HitRecord};
use crate::math::Ray;
use crate::prelude::*;
use crate::texture::{SolidColor, Texture};

pub use dielectric::Dielectric;
pub use diffuse::Diffuse;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub static DEFAULT_MATERIAL: std::sync::LazyLock<Arc<dyn Material>> =
    std::sync::LazyLock::new(|| Arc::new(Diffuse::new(Arc::new(SolidColor::new(ERROR_COLOR)))));

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, hit_record: &HitRecord, hit_data: &HitData) -> Option<(Ray, Color)>;
}
