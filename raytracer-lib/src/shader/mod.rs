use crate::prelude::*;

mod null;
mod hit_struct;

pub use self::null::NullShader;
pub use self::hit_struct::Hit;

pub trait Shader {
	fn get_name(&self) -> &str;
	fn ambient(&self) -> Color;
	fn apply(&self, hit_struct: &Hit) -> Color;
}