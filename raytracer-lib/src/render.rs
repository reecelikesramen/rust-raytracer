use crate::antialias::{antialias, AntialiasMethod};
use crate::scene::Scene;
use crate::shader::Hit;
use crate::Framebuffer;
use crate::{color, prelude::*};
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub fn render(
    scene: &Scene,
    sqrt_rays_per_pixel: u16,
    antialias_method: AntialiasMethod,
    per_pixel_cb: Option<Arc<AtomicUsize>>,
) -> Framebuffer {
    let mut fb = Framebuffer::new(scene.image_width, scene.image_height);
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

    // Create iterator of all pixel coordinates
    let pixels: Vec<(u32, u32)> = (0..width)
        .flat_map(|i| (0..height).map(move |j| (i, j)))
        .collect();

    // Process pixels in parallel
    pixels.par_iter().for_each(|(i, j)| {
        render_pixel(
            fb,
            scene,
            sqrt_rays_per_pixel,
            antialias_method,
            *i,
            *j,
            per_pixel_cb,
            wasm_log,
        );
    });
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
            let hit = Hit::new(ray, &scene);

            color += ray_color(&scene, hit);
        }
    }
    // divide by number of samples
    color /= (sqrt_rays_per_pixel * sqrt_rays_per_pixel) as f32;

    if let Some(counter) = per_pixel_cb {
        counter.fetch_add(1, Ordering::Relaxed);
    }
    fb.set_pixel(i, j, color);
}

fn ray_color<'pixel>(scene: &'pixel Scene, mut hit: Hit<'pixel>) -> Color {
    if hit.depth >= scene.recursion_depth {
        return color!(0.0, 0.0, 0.0);
    }

    if scene.bvh.closest_hit(&mut hit) {
        if let Some((ray, attenuation)) = hit.shape.unwrap().get_material().scatter(&hit) {
            // TODO: can exit early out of this recursion if the total color is significantly near zero
            attenuation.component_mul(&ray_color(&scene, hit.bounce(ray)))
        } else {
            color!(0.0, 0.0, 0.0)
        }
    } else {
        scene.background_color
    }
}
