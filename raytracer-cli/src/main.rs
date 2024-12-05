mod output;
use std::{collections::HashMap, path::Path, sync::Arc, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use output::save;

use clap::{Parser, ValueEnum};
use rayon::prelude::*;
use raytracer_lib::{public_consts, render_pixel, Framebuffer, SceneDescription, SceneGraph};

#[derive(Debug, Clone, ValueEnum)]
enum AntialiasMethod {
    Normal,
    Jittered,
    Random,
}

#[derive(Parser, Debug)]
#[command(author = "Reece Holmdahl", version = None, about="Raytracer CLI", long_about = None)]
struct RayTracerArgs {
    #[arg(short = 'x', long = "width", default_value = None)]
    width: Option<u32>,
    #[arg(short = 'y', long = "height", default_value = None)]
    height: Option<u32>,
    #[arg(short = 'i', long = "scene-path")]
    scene_path: String,
    #[arg(short = 'o', long = "output", default_value = "out.png")]
    output_path: String,
    #[arg(short = 'r', long = "rays-per-pixel", default_value = None)]
    rays_per_pixel: Option<u16>,
    #[arg(short = 'd', long = "recursion-depth", default_value = None)]
    recursion_depth: Option<u16>,
    #[arg(long = "aspect-ratio", default_value = None)]
    aspect_ratio: Option<f64>,
    #[arg(long = "disable-shadows", default_value_t = false)]
    disable_shadows: bool,
    #[arg(long = "render-normals", default_value_t = false)]
    render_normals: bool,
    #[arg(long = "antialias-method", value_enum, default_value = None)]
    antialias_method: Option<AntialiasMethod>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = RayTracerArgs::parse();

    #[cfg(debug_assertions)]
    println!("{:?}", args);

    // Scene parsing spinner
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100)); // Spin every 100ms
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    // Load JSON into scene description
    spinner.set_message("Loading JSON...");
    let scene_json = std::fs::read_to_string(&args.scene_path)?;
    let scene_desc = SceneDescription::from_json(&scene_json)?;

    // Load scene data
    spinner.set_message("Loading scene data...");
    let scene_root = Path::new(&args.scene_path).parent().unwrap();
    let mut scene_data: HashMap<String, Vec<u8>> = HashMap::new();
    for relative_path in &scene_desc.data_needed {
        let bytes = std::fs::read(scene_root.join(relative_path))?;
        scene_data.insert(relative_path.clone(), bytes);
    }

    // Scene parsing
    let scene = SceneGraph::from_description(
        &scene_desc,
        &scene_data,
        args.width,
        args.height,
        args.aspect_ratio,
        args.recursion_depth,
    )?;

    // #[cfg(debug_assertions)]
    // println!("{:#?}", scene);

    spinner.finish_with_message("Scene parsing complete");

    let rays_per_pixel = args
        .rays_per_pixel
        .unwrap_or(public_consts::DEFAULT_RAYS_PER_PIXEL);
    let sqrt_rays_per_pixel = (rays_per_pixel as f64).sqrt() as u16;

    // error if rays_per_pixel is not a perfect square
    if sqrt_rays_per_pixel * sqrt_rays_per_pixel != rays_per_pixel {
        return Err("rays_per_pixel must be a perfect square".into());
    }

    let width = scene.image_width;
    let height = scene.image_height;
    let pb = indicatif::ProgressBar::new((width * height) as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{wide_bar} {percent}%\n[{elapsed_precise}] [{eta_precise}]")?,
        // .template("Elapsed: {elapsed} Remaining: {eta}\n{wide_bar} {percent}%")?,
    );

    let antialias_method = match args.antialias_method {
        Some(AntialiasMethod::Normal) => raytracer_lib::AntialiasMethod::Normal,
        Some(AntialiasMethod::Jittered) => raytracer_lib::AntialiasMethod::Jittered,
        Some(AntialiasMethod::Random) => raytracer_lib::AntialiasMethod::Random,
        None => raytracer_lib::AntialiasMethod::Normal,
    };

    let framebuffer = Arc::new(Framebuffer::new(width, height));

    // # Column-chunk parallel version 37 seconds
    (0..width).into_par_iter().for_each(|i| {
        for j in 0..height {
            render_pixel(
                framebuffer.clone(),
                &scene,
                sqrt_rays_per_pixel,
                antialias_method,
                i,
                j,
            );
            pb.inc(1);
        }
    });

    let framebuffer = Arc::try_unwrap(framebuffer).map_err(|_| "Failed to unwrap framebuffer")?;

    pb.finish_with_message("Rendering complete");
    save(args.output_path.as_str(), framebuffer)?;

    Ok(())
}
