use crate::prelude::*;
use color;

#[derive(Clone)]
struct Framebuffer {
	width: u32,
	height: u32,
	pixels: Vec<Color>
}

impl Framebuffer {
	// empty framebuffer
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width,
			height,
			pixels: vec![color!(0.0, 0.0, 0.0); (width * height) as usize]
		}
	}

	// pixel index to framebuffer index
	fn index(&self, i: u32, j: u32) -> usize {
		(i + j * self.width) as usize
	}

	// set pixel
	pub fn set_pixel(&mut self, i: u32, j: u32, color: Color) {
		let idx = self.index(i, j);
		self.pixels[idx] = color;
	}

	// clear color
	pub fn clear_color(&mut self, color: Color) {
		for pixel in self.pixels.iter_mut() {
			*pixel = color;
		}
	}
}