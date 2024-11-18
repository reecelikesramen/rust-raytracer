use crate::antialias::{antialias, AntialiasMethod};
use crate::scene::Scene;
use crate::shader::Hit;
use crate::Framebuffer;
use crate::{color, prelude::*};

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

    for i in 0..width {
        for j in 0..height {
            render_pixel(
                fb,
                scene,
                sqrt_rays_per_pixel,
                antialias_method,
                i,
                j,
                per_pixel_cb,
                wasm_log,
            )
            // wasm_log(&format!("On pixel {} {}", i, j));
        }
    }
}

pub fn render_pixel(
    fb: &mut Framebuffer,
    scene: &Scene,
    sqrt_rays_per_pixel: u16,
    antialias_method: AntialiasMethod,
    i: u32,
    j: u32,
    per_pixel_cb: Option<&dyn Fn() -> ()>,
    wasm_log: Option<&dyn Fn(&str) -> ()>,
) {
    let mut color = color!(0.0, 0.0, 0.0);
    for p in 0..sqrt_rays_per_pixel {
        for q in 0..sqrt_rays_per_pixel {
            let (di, dj) = antialias(antialias_method, sqrt_rays_per_pixel, p, q);
            let ray = scene.camera.generate_ray(i, j, di, dj);
            let mut hit = Hit::new(ray, &scene);

            if scene.bvh.closest_hit(&mut hit) {
                color += hit.shape.unwrap().get_shader().apply(&hit);
            } else {
                color += scene.background_color;
            }
        }
    }
    // divide by number of samples
    color /= (sqrt_rays_per_pixel * sqrt_rays_per_pixel) as f32;

    if let Some(cb) = per_pixel_cb {
        cb();
    }
    fb.set_pixel(i, j, color);
}
