#![allow(unused)]

extern crate approx;
extern crate nalgebra;
extern crate serde;

use std::{
    collections::{hash_map, HashMap},
    hash::Hash,
};

use crate::prelude::*;

mod camera;
mod constants;
mod framebuffer;
mod geometry;
mod math;
mod prelude;
mod render;
mod scene;
mod shader;

use camera::Camera;
pub use framebuffer::Framebuffer;
pub use render::render;
pub use scene::load_scene;
pub use scene::Scene;
use shader::{LambertianShader, Shader};
