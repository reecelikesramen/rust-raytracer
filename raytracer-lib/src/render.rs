use std::sync::Arc;

use crate::antialias::{antialias, AntialiasMethod};
use crate::scene::Scene;
use crate::shader::Hit;
use crate::Framebuffer;
use crate::{color, prelude::*};

pub fn render_pixel(
    fb: Arc<Framebuffer>,
    scene: &Scene,
    sqrt_rays_per_pixel: u16,
    antialias_method: AntialiasMethod,
    i: u32,
    j: u32,
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
