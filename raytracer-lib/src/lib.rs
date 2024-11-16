#![allow(unused)]

extern crate approx;
extern crate nalgebra;
extern crate serde;

use std::{
    collections::{hash_map, HashMap},
    hash::Hash,
};

use crate::prelude::*;

mod antialias;
mod camera;
mod framebuffer;
mod geometry;
mod light;
mod math;
mod prelude;
mod render;
mod scene;
mod shader;

pub use antialias::AntialiasMethod;
pub use framebuffer::Framebuffer;
pub use prelude::public_consts;
pub use render::{render, render_mut};
pub use scene::parse_scene;
pub use scene::Scene;
