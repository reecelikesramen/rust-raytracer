mod output;
use std::{path::Path, sync::Arc};

use output::save;

use clap::{Parser, ValueEnum};
use rayon::prelude::*;
use raytracer_lib::{parse_scene, public_consts, render_pixel, Framebuffer};

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

    let width = scene.image_width;
    let height = scene.image_height;
    let pb = indicatif::ProgressBar::new((width * height) as u64);
    pb.set_style(indicatif::ProgressStyle::default_bar().template("{wide_bar} {percent}% ")?);

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

    let framebuffer = match Arc::try_unwrap(framebuffer) {
        Ok(fb) => fb,
        Err(_) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to unwrap framebuffer after render",
            )))
        }
    };

    save(args.output_path.as_str(), framebuffer);
    pb.finish_with_message("Render complete");

    Ok(())
}

pub fn split_image_into_grid(width: u32, height: u32, grid_size: u32) -> Vec<Vec<(u32, u32)>> {
    let mut grid_cells = Vec::new();

    let complete_cols = width / grid_size;
    let complete_rows = height / grid_size;
    let remainder_width = width % grid_size;
    let remainder_height = height % grid_size;

    // Pre-calculate capacity to avoid reallocations
    let total_cells = (complete_cols + (remainder_width > 0) as u32)
        * (complete_rows + (remainder_height > 0) as u32);
    grid_cells.reserve_exact(total_cells as usize);

    // Process complete grid cells
    for row in 0..complete_rows {
        for col in 0..complete_cols {
            let mut cell = Vec::with_capacity((grid_size * grid_size) as usize);
            for i in 0..grid_size {
                for j in 0..grid_size {
                    cell.push((col * grid_size + j, row * grid_size + i));
                }
            }
            grid_cells.push(cell);
        }
    }

    // Process right edge
    if remainder_width > 0 {
        for row in 0..complete_rows {
            let mut cell = Vec::with_capacity((grid_size * remainder_width) as usize);
            for i in 0..grid_size {
                for j in 0..remainder_width {
                    cell.push((complete_cols * grid_size + j, row * grid_size + i));
                }
            }
            grid_cells.push(cell);
        }
    }

    // Process bottom edge
    if remainder_height > 0 {
        for col in 0..complete_cols {
            let mut cell = Vec::with_capacity((remainder_height * grid_size) as usize);
            for i in 0..remainder_height {
                for j in 0..grid_size {
                    cell.push((col * grid_size + j, complete_rows * grid_size + i));
                }
            }
            grid_cells.push(cell);
        }
    }

    // Process bottom-right corner
    if remainder_width > 0 && remainder_height > 0 {
        let mut cell = Vec::with_capacity((remainder_height * remainder_width) as usize);
        for i in 0..remainder_height {
            for j in 0..remainder_width {
                cell.push((complete_cols * grid_size + j, complete_rows * grid_size + i));
            }
        }
        grid_cells.push(cell);
    }

    grid_cells
}
