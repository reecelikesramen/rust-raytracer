extern crate nalgebra as na;
extern crate serde;

use crate::prelude::*;

mod antialias;
mod camera;
mod framebuffer;
mod geometry;
mod hit_record;
mod material;
mod math;
mod prelude;
mod render;
mod scene;
mod texture;

pub use antialias::AntialiasMethod;
pub use framebuffer::Framebuffer;
pub use prelude::{public_consts, Real};
pub use render::render_pixel;
pub use scene::{SceneDescription, SceneGraph};
