use crate::prelude::*;

mod blinn_phong;
mod ggx_mirror;
mod hit_struct;
mod lambertian;
mod normal;
mod null;
mod perfect_mirror;

pub use blinn_phong::BlinnPhongShader;
pub use ggx_mirror::GGXMirrorShader;
pub use hit_struct::Hit;
pub use lambertian::LambertianShader;
pub use normal::NormalShader;
pub use null::NullShader;
pub use perfect_mirror::PerfectMirrorShader;

pub trait Shader: Send + Sync + std::fmt::Debug {
    fn apply(&self, hit: &Hit) -> Color;
}
