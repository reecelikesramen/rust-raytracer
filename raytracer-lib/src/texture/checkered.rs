use crate::color;

use super::*;

#[derive(Debug)]
pub struct CheckeredTexture {
    color1: Color,
    color2: Color,
    inv_scale: Real,
}

impl CheckeredTexture {
    pub fn new(color1: Color, color2: Color, scale: Real) -> Self {
        Self {
            color1,
            color2,
            inv_scale: 1.0 / scale,
        }
    }
}

impl Default for CheckeredTexture {
    fn default() -> Self {
        Self {
            color1: Color::new(1.0, 1.0, 1.0),
            color2: Color::new(0.0, 0.0, 0.0),
            inv_scale: 64.0,
        }
    }
}

impl Texture for CheckeredTexture {
    fn color(&self, uv: (Real, Real), _p: &P3) -> Color {
        // let x_int = (p.x * self.inv_scale).floor() as i32;
        // let y_int = (p.y * self.inv_scale).floor() as i32;
        // let z_int = (p.z * self.inv_scale).floor() as i32;
        // if (x_int + y_int + z_int) % 2 == 0 {
        //     self.color1
        // } else {
        //     self.color2
        // }

        let x_int = (uv.0 * self.inv_scale).floor() as i32;
        let y_int = (uv.1 * self.inv_scale).floor() as i32;
        if (x_int + y_int) % 2 == 0 {
            self.color1
        } else {
            self.color2
        }
    }
}
