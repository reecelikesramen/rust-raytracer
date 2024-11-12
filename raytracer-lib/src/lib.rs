#![allow(unused)]

extern crate approx;
extern crate nalgebra;

use std::{
    collections::{hash_map, HashMap},
    hash::Hash,
};

use crate::prelude::*;

mod camera;
mod framebuffer;
mod geometry;
mod math;
mod prelude;
mod render;
mod scene;
mod shader;

pub fn example_scene(px: u32, py: u32) -> Scene<'static> {
    let mut camera = Box::new(camera::PerspectiveCamera::new(
        vec3!(0.0, 0.0, 0.0),
        &vec3!(0.0, 0.0, -1.0),
        1.0,
        1.0,
    ));
		camera.set_image_pixels(px, py);
    Scene {
        camera,
        shapes: vec![Box::new(geometry::Sphere::new(vec3!(0.0, 0.0, -5.0), 1.0))],
        shaders: HashMap::default(),
    }
}

use camera::Camera;
pub use framebuffer::Framebuffer;
pub use render::render;
pub use scene::Scene;
use shader::Shader;
