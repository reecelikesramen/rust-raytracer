use crate::prelude::*;

mod blinn_phong;
mod hit_struct;
mod lambertian;
mod null;

pub use self::blinn_phong::BlinnPhongShader;
pub use self::hit_struct::Hit;
pub use self::lambertian::LambertianShader;
pub use self::null::NullShader;

pub trait Shader: Send + Sync + std::fmt::Debug {
    fn apply(&self, hit: &Hit) -> Color;
}
