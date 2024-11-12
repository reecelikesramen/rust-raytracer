#![allow(unused)] // For beginning only

mod output;
use output::save;

extern crate clap;
extern crate indicatif;
extern crate raytracer_lib;
use clap::Parser;
use raytracer_lib::{example_scene, render, Framebuffer, Scene};

#[derive(Parser, Debug)]
#[command(author = "Reece Holmdahl", version = None, about="Raytracer CLI", long_about = None)]
struct RayTracerArgs {
    #[arg(short = 'x', long = "width", default_value_t = 360)]
    width: u32,
    #[arg(short = 'y', long = "height", default_value_t = 360)]
    height: u32,
    #[arg(short = 'o', long = "output", default_value = "out.png")]
    output_path: String,
    #[arg(short = 'r', long = "rays-per-pixel", default_value_t = 4)]
    rays_per_pixel: u16,
    #[arg(short = 'd', long = "recursion-depth", default_value_t = 4)]
    recursion_depth: u16,
}

fn main() {
    let args = RayTracerArgs::parse();
    println!("{:?}", args);
    let scene: Scene = example_scene(args.width, args.height);
    let pb = indicatif::ProgressBar::new(args.width as u64 * args.height as u64);
    let per_pixel_cb = || { pb.inc(1); };
    let fb = render(
        &scene,
        args.width,
        args.height,
        args.rays_per_pixel,
        args.recursion_depth,
        Some(&per_pixel_cb),
    );
    save(args.output_path.as_str(), &fb);
    pb.finish_with_message("Render complete");
}
