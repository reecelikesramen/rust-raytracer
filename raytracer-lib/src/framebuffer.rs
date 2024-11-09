use Vec3f;

struct Framebuffer {
	width: u32,
	height: u32,
	pixels: Vec<Vec3f>
}

impl Framebuffer {
	// empty framebuffer
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width,
			height,
			pixels: vec![Vec3f::new(0.0, 0.0, 0.0); (width * height) as usize]
		}
	}

	// copy from another framebuffer
	pub fn copy(other: &Framebuffer) -> Self {
		Self {
			width: other.width,
			height: other.height,
			pixels: other.pixels.clone()
		}
	}

	// pixel index to framebuffer index
	fn index(&self, i: u32, j: u32) -> usize {
		(i + j * self.width) as usize
	}

	// set pixel
	pub fn set_pixel(&mut self, i: u32, j: u32, color: Vec3f) {
		let idx = self.index(i, j);
		self.pixels[idx] = color;
	}

	// clear color
	pub fn clear_color(&mut self, color: Vec3f) {
		for pixel in self.pixels.iter_mut() {
			*pixel = color;
		}
	}
}