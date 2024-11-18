use crate::prelude::*;

mod blinn_phong;
mod ggx_mirror;
mod hit_struct;
mod lambertian;
mod normal;
mod null;
mod perfect_mirror;

pub use self::blinn_phong::BlinnPhongShader;
pub use self::ggx_mirror::GGXMirrorShader;
pub use self::hit_struct::Hit;
pub use self::lambertian::LambertianShader;
pub use self::normal::NormalShader;
pub use self::perfect_mirror::PerfectMirrorShader;

pub trait Shader: Send + Sync + std::fmt::Debug {
    fn apply(&self, hit: &Hit) -> Color;
}
