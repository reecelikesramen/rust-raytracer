mod dielectric;
mod diffuse;
mod diffuse_light;
mod lambertian;
mod metal;

use std::fmt::Debug;
use std::sync::Arc;

use crate::hit_record::{HitData, HitRecord};
use crate::math::Ray;
use crate::texture::{SolidColor, Texture};
use crate::{color, prelude::*};

pub use dielectric::Dielectric;
pub use diffuse::Diffuse;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub static DEFAULT_MATERIAL: std::sync::LazyLock<Arc<dyn Material>> =
    std::sync::LazyLock::new(|| Arc::new(Diffuse::new(Arc::new(SolidColor::new(ERROR_COLOR)))));

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, _hit_record: &HitRecord, _hit_data: &HitData) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, _uv: (Real, Real), _point: &P3) -> Color {
        color!(0.0, 0.0, 0.0)
    }
}
