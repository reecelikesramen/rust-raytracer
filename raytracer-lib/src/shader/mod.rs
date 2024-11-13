use crate::prelude::*;

mod null;
mod hit_struct;
mod lambertian;

pub use self::null::NullShader;
pub use self::lambertian::LambertianShader;
pub use self::hit_struct::Hit;

pub trait Shader {
	fn get_name(&self) -> &str;
	fn ambient(&self) -> Color;
	fn apply(&self, hit: &Hit) -> Color;
}