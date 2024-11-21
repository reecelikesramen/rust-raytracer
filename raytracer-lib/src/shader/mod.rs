use crate::prelude::*;

mod blinn_phong;
mod diffuse;
mod ggx_mirror;
mod hit_record;
mod lambertian;
mod normal;
mod null;
mod perfect_mirror;

pub use blinn_phong::BlinnPhongShader;
pub use diffuse::DiffuseShader;
pub use ggx_mirror::GGXMirrorShader;
pub use hit_record::Hit;
pub use lambertian::LambertianShader;
pub use normal::NormalShader;
pub use null::NullShader;
pub use perfect_mirror::PerfectMirrorShader;

pub trait Shader: Send + Sync + std::fmt::Debug {
    fn apply(&self, hit: &Hit) -> Color;
}
