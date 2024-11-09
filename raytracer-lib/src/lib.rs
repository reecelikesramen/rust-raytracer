#![allow(unused)]

extern crate nalgebra;
extern crate approx;

use crate::prelude::*;

mod prelude;
mod math;
mod camera;
mod framebuffer;

pub use framebuffer::Framebuffer;