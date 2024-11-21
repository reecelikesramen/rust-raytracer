mod output;
use std::path::Path;

use output::save;

extern crate clap;
extern crate indicatif;
extern crate raytracer_lib;
use clap::{Parser, ValueEnum};
use raytracer_lib::{parse_scene, public_consts, render};

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

    // read scene path as string
    let scene_json = std::fs::read_to_string(&args.scene_path)?;
    let scene_data_path = Path::new(&args.scene_path)
        .parent()
        .unwrap()
        .to_str()
        .unwrap();

    let scene = parse_scene(
        &scene_json,
        &scene_data_path,
        args.width,
        args.height,
        args.aspect_ratio,
        args.recursion_depth,
        args.disable_shadows,
        args.render_normals,
    )?;

    // #[cfg(debug_assertions)]
    // println!("{:#?}", scene);

    let rays_per_pixel = args
        .rays_per_pixel
        .unwrap_or(public_consts::DEFAULT_RAYS_PER_PIXEL);
    let sqrt_rays_per_pixel = (rays_per_pixel as f64).sqrt() as u16;

    // error if rays_per_pixel is not a perfect square
    if sqrt_rays_per_pixel * sqrt_rays_per_pixel != rays_per_pixel {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "rays_per_pixel must be a perfect square",
        )));
    }

    let pb = indicatif::ProgressBar::new((scene.image_width * scene.image_height) as u64);

    pb.set_style(indicatif::ProgressStyle::default_bar().template("{wide_bar} {percent}% ")?);

    let per_pixel_cb = || {
        pb.inc(1);
    };

    let aa_method = match args.antialias_method {
        Some(AntialiasMethod::Normal) => raytracer_lib::AntialiasMethod::Normal,
        Some(AntialiasMethod::Jittered) => raytracer_lib::AntialiasMethod::Jittered,
        Some(AntialiasMethod::Random) => raytracer_lib::AntialiasMethod::Random,
        None => raytracer_lib::AntialiasMethod::Normal,
    };

    let fb = render(&scene, sqrt_rays_per_pixel, aa_method, Some(&per_pixel_cb));
    save(args.output_path.as_str(), fb);
    pb.finish_with_message("Render complete");

    Ok(())
}
