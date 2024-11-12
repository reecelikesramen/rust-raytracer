use std::borrow::BorrowMut;
use std::any::Any;

use crate::geometry::Sphere;
use crate::{prelude::*, scene};
use crate::scene::Scene;
use crate::shader::Hit;
use crate::Framebuffer;
use color;

static NOP_CB: fn() -> () = || {};

pub fn render<'scene>(scene: &'scene Scene, width: u32, height: u32, rays_per_pixel: u16, recursion_depth: u16, per_pixel_cb: Option<&dyn Fn() -> ()>) -> Framebuffer {
	let cb = per_pixel_cb.unwrap_or(&NOP_CB);
	let mut fb = Framebuffer::new(width, height);
	for i in 0..width {
		for j in 0..height {
			// TODO: implement rpp and anti-aliasing
			let mut any_hit = false;
			let mut color = color!(0.0, 0.0, 0.0);
			let (di, dj) = (0.5, 0.5);
			let ray = scene.camera.generate_ray(i, j, di, dj);
			let mut hit = Hit::new();

			// TODO: scene.closestHit
			for shape in &scene.shapes {
				if shape.closest_hit(&ray, &mut hit) {
					// print!("hello");
					// if let Some(s) = hit.shape {
					// 	if let Some(sp) = (&s as &dyn Any).downcast_ref::<Sphere>() {
					// 		print!("sphere!")
					// 	}
					// }
					color += hit.shape.unwrap().get_shader().apply(&hit)
				}
			}
			
			cb();
			fb.set_pixel(i, j, color);
		}
	}

	fb
}
