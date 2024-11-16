use std::str::FromStr;

use raytracer_lib::{parse_scene, public_consts, render_mut, Framebuffer};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Debug)]
pub struct RayTracerArgs {
    width: u32,
    height: u32,
    #[serde(default)]
    rays_per_pixel: Option<u16>,
    #[serde(default)]
    recursion_depth: Option<u16>,
    #[serde(default)]
    aspect_ratio: Option<f64>,
    #[serde(default)]
    disable_shadows: bool,
    #[serde(default)]
    render_normals: bool,
    #[serde(default)]
    antialias_method: Option<String>,
}

#[wasm_bindgen]
pub fn render_wasm(fb: *mut Framebuffer, scene_data: String, args: JsValue) -> Result<(), JsValue> {
    let args: RayTracerArgs = serde_wasm_bindgen::from_value(args)?;

    let scene = parse_scene(
        &scene_data,
        Some(args.width),
        Some(args.height),
        args.aspect_ratio,
        args.recursion_depth,
        args.disable_shadows,
        args.render_normals,
    )
    .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let rays_per_pixel = args
        .rays_per_pixel
        .unwrap_or(public_consts::DEFAULT_RAYS_PER_PIXEL);
    let sqrt_rays_per_pixel = (rays_per_pixel as f64).sqrt() as u16;

    // error if rays_per_pixel is not a perfect square
    if sqrt_rays_per_pixel * sqrt_rays_per_pixel != rays_per_pixel {
        return Err(JsValue::from_str("rays_per_pixel must be a perfect square"));
    }

    let antialias_method = match args.antialias_method {
        Some(s) => raytracer_lib::AntialiasMethod::from_str(s.as_str()).unwrap(),
        _ => raytracer_lib::AntialiasMethod::Normal,
    };

    let framebuffer: &mut Framebuffer = unsafe { &mut *fb };

    render_mut(
        framebuffer,
        &scene,
        sqrt_rays_per_pixel,
        antialias_method,
        None,
    );

    Ok(())
}
