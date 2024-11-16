use std::any::Any;
use std::borrow::BorrowMut;

use crate::antialias::{antialias, AntialiasMethod};
use crate::geometry::Sphere;
use crate::scene::Scene;
use crate::shader::Hit;
use crate::Framebuffer;
use crate::{color, prelude::*, scene};

static NOP_CB: fn() -> () = || {};

pub fn render(
    scene: &Scene,
    sqrt_rays_per_pixel: u16,
    antialias_method: AntialiasMethod,
    per_pixel_cb: Option<&dyn Fn() -> ()>,
) -> Framebuffer {
    let mut fb = Framebuffer::new(scene.image_width, scene.image_width);
    render_mut(
        &mut fb,
        scene,
        sqrt_rays_per_pixel,
        antialias_method,
        per_pixel_cb,
        None,
    );
    fb
}

pub fn render_mut(
    fb: &mut Framebuffer,
    scene: &Scene,
    sqrt_rays_per_pixel: u16,
    antialias_method: AntialiasMethod,
    per_pixel_cb: Option<&dyn Fn() -> ()>,
    wasm_log: Option<&dyn Fn(&str) -> ()>,
) {
    let width = scene.image_width;
    let height = scene.image_height;
    let cb = per_pixel_cb.unwrap_or(&NOP_CB);
    let wasm_log = wasm_log.unwrap_or(&|_| {});

    for i in 0..width {
        for j in 0..height {
            let mut any_hit = false;
            let mut color = color!(0.0, 0.0, 0.0);
            for p in 0..sqrt_rays_per_pixel {
                for q in 0..sqrt_rays_per_pixel {
                    let (di, dj) = antialias(antialias_method, sqrt_rays_per_pixel, p, q);
                    let ray = scene.camera.generate_ray(i, j, di, dj);
                    let mut hit = Hit::new(ray, &scene);

                    if scene.bvh.closest_hit(&mut hit) {
                        any_hit = true;
                        color += hit.shape.unwrap().get_shader().apply(&hit);
                    } else {
                        color += scene.background_color;
                    }
                }
            }
            // divide by number of samples
            color /= (sqrt_rays_per_pixel * sqrt_rays_per_pixel) as f32;

            cb();
            fb.set_pixel(i, j, color);
            // wasm_log(&format!("On pixel {} {}", i, j));
        }
    }
}
