extern crate nalgebra as na;
extern crate serde;

use crate::prelude::*;

mod antialias;
mod camera;
mod framebuffer;
mod geometry;
mod light;
mod material;
mod math;
mod prelude;
mod render;
mod scene;
mod shader;

pub use antialias::AntialiasMethod;
pub use framebuffer::Framebuffer;
pub use prelude::public_consts;
pub use prelude::Real;
pub use render::render_pixel;
pub use scene::parse_scene;
pub use scene::Scene;
