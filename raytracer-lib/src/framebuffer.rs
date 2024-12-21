use crate::prelude::*;
use std::sync::RwLock;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: RwLock<Vec<[f32; 3]>>,
    pub samples: RwLock<Vec<u32>>,
}

impl Framebuffer {
    // empty framebuffer
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: RwLock::new(vec![[0.0, 0.0, 0.0]; (width * height) as usize]),
            samples: RwLock::new(vec![0; (width * height) as usize]),
        }
    }

    // pixel index to framebuffer index
    #[inline]
    fn index(&self, i: u32, j: u32) -> usize {
        (i + j * self.width) as usize
    }

    pub fn add_samples(&self, i: u32, j: u32, color: Color, samples: u32) {
        let idx = self.index(i, j);
        if let Ok(mut pixels_lock) = self.pixels.write() {
            if let Ok(mut samples_lock) = self.samples.write() {
                pixels_lock[idx][0] += color[0];
                pixels_lock[idx][1] += color[1];
                pixels_lock[idx][2] += color[2];
                samples_lock[idx] += samples;
            }
        }
    }

    pub fn get_pixels(&self) -> Vec<[f32; 3]> {
        let pixels_guard = self.pixels.read().expect("Failed to lock pixels");
        let samples_guard = self.samples.read().expect("Failed to lock samples");
        // zip together pixels and samples
        pixels_guard
            .iter()
            .zip(samples_guard.iter())
            .map(|(p, &s)| [p[0] / s as f32, p[1] / s as f32, p[2] / s as f32])
            .collect()
    }
}
