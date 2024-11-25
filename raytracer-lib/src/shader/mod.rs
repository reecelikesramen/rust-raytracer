use crate::{hit_record::HitRecord, prelude::*};

mod blinn_phong;
mod diffuse;
mod ggx_mirror;
mod lambertian;
mod normal;
mod null;
mod perfect_mirror;

pub use blinn_phong::BlinnPhongShader;
pub use diffuse::DiffuseShader;
pub use ggx_mirror::GGXMirrorShader;
pub use lambertian::LambertianShader;
pub use normal::NormalShader;
pub use null::NullShader;
pub use perfect_mirror::PerfectMirrorShader;

pub trait Shader: Send + Sync + std::fmt::Debug {
    fn apply(&self, hit: &HitRecord) -> Color;
}
