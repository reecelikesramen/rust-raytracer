use crate::{color, prelude::*};
use std::sync::RwLock;

pub struct Framebuffer {
    width: u32,
    height: u32,
    pixels: RwLock<Vec<[f32; 3]>>,
}

impl Framebuffer {
    // empty framebuffer
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: RwLock::new(vec![[0.0, 0.0, 0.0]; (width * height) as usize]),
        }
    }

    // pixel index to framebuffer index
    fn index(&self, i: u32, j: u32) -> usize {
        (i + j * self.width) as usize
    }

    // set pixel - thread safe
    pub fn set_pixel(&self, i: u32, j: u32, color: Color) {
        let idx = self.index(i, j);
        if let Ok(mut pixels) = self.pixels.write() {
            pixels[idx][0] = color[0];
            pixels[idx][1] = color[1];
            pixels[idx][2] = color[2];
        }
    }

    // get pixel - thread safe
    pub fn get_pixel(&self, i: u32, j: u32) -> Color {
        let idx = self.index(i, j);
        if let Ok(pixels) = self.pixels.read() {
            color!(
                pixels[idx][0],
                pixels[idx][1],
                pixels[idx][2]
            )
        } else {
            color!(0.0, 0.0, 0.0) // fallback in case of lock poisoning
        }
    }

    pub fn get_pixels(&self) -> Vec<[f32; 3]> {
        self.pixels.read()
            .map(|pixels| pixels.clone())
            .unwrap_or_else(|_| vec![[0.0, 0.0, 0.0]; (self.width * self.height) as usize])
    }

    // clear color - thread safe
    pub fn clear_color(&self, color: Color) {
        if let Ok(mut pixels) = self.pixels.write() {
            for pixel in pixels.iter_mut() {
                pixel[0] = color[0];
                pixel[1] = color[1];
                pixel[2] = color[2];
            }
        }
    }

    // Get dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    // Get raw pixels for final output
    pub fn into_raw(self) -> Vec<[f32; 3]> {
        self.pixels.into_inner()
            .unwrap_or_else(|_| vec![[0.0, 0.0, 0.0]; (self.width * self.height) as usize])
    }

    // Create framebuffer from raw pixels
    pub fn from_raw(width: u32, height: u32, pixels: Vec<[f32; 3]>) -> Self {
        assert_eq!(pixels.len(), (width * height) as usize);
        Self {
            width,
            height,
            pixels: RwLock::new(pixels),
        }
    }
}
