use crate::prelude::*;

mod hit_struct;
mod lambertian;
mod null;

pub use self::hit_struct::Hit;
pub use self::lambertian::LambertianShader;
pub use self::null::NullShader;

pub trait Shader: Send + Sync + std::fmt::Debug {
    fn get_name(&self) -> &str;
    fn ambient(&self) -> Color;
    fn apply(&self, hit: &Hit) -> Color;
}
