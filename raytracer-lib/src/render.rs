use std::sync::Arc;

use crate::antialias::{antialias, AntialiasMethod};
use crate::hit_record::HitRecord;
use crate::scene::Scene;
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
            let hit = HitRecord::new(ray, &scene);

            color += ray_color(&scene, hit);
        }
    }
    // divide by number of samples
    color /= (sqrt_rays_per_pixel * sqrt_rays_per_pixel) as f32;
    fb.set_pixel(i, j, color);
}

fn ray_color<'pixel>(scene: &'pixel Scene, mut hit: HitRecord<'pixel>) -> Color {
    if hit.depth >= scene.recursion_depth {
        return color!(0.0, 0.0, 0.0);
    }

    let hit_data = match scene.bvh.get_closest_hit_data(&mut hit) {
        Some(hit_data) => hit_data,
        None => return scene.background_color,
    };

    let (ray, attenuation) = match hit_data.material.scatter(&hit, &hit_data) {
        Some((r, a)) => (r, a),
        None => return color!(0.0, 0.0, 0.0),
    };

    attenuation.component_mul(&ray_color(&scene, hit.bounce(ray)))
}
