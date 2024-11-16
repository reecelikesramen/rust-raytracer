use crate::{color, prelude::*};

#[derive(Clone)]
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<[f32; 3]>,
}

impl Framebuffer {
    // empty framebuffer
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![[0.0, 0.0, 0.0]; (width * height) as usize],
        }
    }

    // pixel index to framebuffer index
    fn index(&self, i: u32, j: u32) -> usize {
        (i + j * self.width) as usize
    }

    // set pixel
    pub fn set_pixel(&mut self, i: u32, j: u32, color: Color) {
        let idx = self.index(i, j);
        self.pixels[idx][0] = color[0];
        self.pixels[idx][1] = color[1];
        self.pixels[idx][2] = color[2];
    }

    // get pixel
    pub fn get_pixel(&self, i: u32, j: u32) -> Color {
        let idx = self.index(i, j);
        color!(
            self.pixels[idx][0],
            self.pixels[idx][1],
            self.pixels[idx][2]
        )
    }

    // clear color
    pub fn clear_color(&mut self, color: Color) {
        for pixel in self.pixels.iter_mut() {
            pixel[0] = color[0];
            pixel[1] = color[1];
            pixel[2] = color[2];
        }
    }
}
