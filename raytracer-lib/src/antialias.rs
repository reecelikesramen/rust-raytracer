use rand::Rng;

use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum AntialiasMethod {
    Normal,
    Jittered,
    Random,
}

impl std::str::FromStr for AntialiasMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(AntialiasMethod::Normal),
            "jittered" => Ok(AntialiasMethod::Jittered),
            "random" => Ok(AntialiasMethod::Random),
            _ => Err(()),
        }
    }
}

pub fn antialias(
    method: AntialiasMethod,
    sqrt_rays_per_pixel: u16,
    p: u16,
    q: u16,
) -> (Real, Real) {
    match method {
        AntialiasMethod::Normal => normal(sqrt_rays_per_pixel, p, q),
        AntialiasMethod::Jittered => jittered(sqrt_rays_per_pixel, p, q),
        AntialiasMethod::Random => random(sqrt_rays_per_pixel, p, q),
    }
}

fn normal(sqrt_rays_per_pixel: u16, p: u16, q: u16) -> (Real, Real) {
    (
        (p as Real + 0.5) / sqrt_rays_per_pixel as Real,
        (q as Real + 0.5) / sqrt_rays_per_pixel as Real,
    )
}

fn jittered(sqrt_rays_per_pixel: u16, p: u16, q: u16) -> (Real, Real) {
    (
        (p as Real + rand::thread_rng().gen::<Real>()) / sqrt_rays_per_pixel as Real,
        (q as Real + rand::thread_rng().gen::<Real>()) / sqrt_rays_per_pixel as Real,
    )
}

fn random(sqrt_rays_per_pixel: u16, p: u16, q: u16) -> (Real, Real) {
    (
        rand::thread_rng().gen::<Real>(),
        rand::thread_rng().gen::<Real>(),
    )
}
