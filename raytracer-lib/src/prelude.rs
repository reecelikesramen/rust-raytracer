//! Crate prelude

#[cfg(all(feature = "f32", feature = "f64"))]
compile_error!("Features `f32` and `f64` are mutually exclusive. Enable only one.");

#[cfg(not(any(feature = "f32", feature = "f64")))]
compile_error!("Feature `f32` or `f64` must be enabled.");

// full precision
#[cfg(feature = "f64")]
pub type Real = f64;

// half precision
#[cfg(feature = "f32")]
pub type Real = f32;

pub type Color = nalgebra::Vector3<f32>;
pub type Vec3 = nalgebra::Vector3<Real>;

/// Wrapper for types to implement custom behavior
#[derive(Debug)]
pub struct W<T>(pub T);

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3::new($x, $y, $z)
    };
}

#[macro_export]
macro_rules! color {
    ($x:expr, $y:expr, $z:expr) => {
        Color::new($x, $y, $z)
    };
}

pub(crate) use constants::*;
pub use public_consts::*;

mod constants {
    use crate::prelude::*;

    pub(crate) static INFINITY: Real = Real::INFINITY;
    pub(crate) static DEFAULT_IMAGE_PLANE_WIDTH: Real = 0.5;
    pub(crate) static ERROR_COLOR: Color = color!(1.0, 0.0, 1.0);
    pub(crate) static DEFAULT_BACKGROUND_COLOR: Color = color!(0.198, 0.198, 0.198);
    pub(crate) static VERY_SMALL_NUMBER: Real = 1e-6;
    pub(crate) static DEFAULT_CAMERA: &str = "main";

    #[cfg(feature = "f64")]
    pub(crate) static PI: Real = std::f64::consts::PI;
    #[cfg(feature = "f32")]
    pub(crate) static PI: Real = std::f32::consts::PI;
}

pub mod public_consts {
    use crate::AntialiasMethod;

    pub static DEFAULT_IMAGE_WIDTH: u32 = 360;
    pub static DEFAULT_IMAGE_HEIGHT: u32 = 360;
    pub static DEFAULT_RAYS_PER_PIXEL: u16 = 4;
    pub static DEFAULT_RECURSION_DEPTH: u16 = 3;
    pub static DEFAULT_ANTIALIAS_METHOD: AntialiasMethod = AntialiasMethod::Normal;
}
