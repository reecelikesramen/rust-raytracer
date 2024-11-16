//! Crate prelude

// full precision
#[cfg(feature = "f64")]
pub type Real = f64;

// half precision
#[cfg(feature = "f32")]
pub type Real = f32;

pub type Color = nalgebra::Vector3<f32>;
pub type Vec3 = nalgebra::Vector3<Real>;

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

pub use constants::*;

mod constants {
    use crate::prelude::*;

    pub(crate) static INFINITY: Real = Real::INFINITY;
    pub(crate) static DEFAULT_IMAGE_PLANE_WIDTH: Real = 0.5;
    pub(crate) static DEFAULT_ASPECT_RATIO: Real = 0.5;
    pub(crate) static ERROR_COLOR: Color = color!(1.0, 0.0, 1.0);
    pub(crate) static DEFAULT_BACKGROUND_COLOR: Color = color!(0.198, 0.198, 0.198);
    pub(crate) static VERY_SMALL_NUMBER: Real = 1e-6;

    #[cfg(feature = "f64")]
    pub(crate) static PI: Real = std::f64::consts::PI;
    #[cfg(feature = "f32")]
    pub(crate) static PI: Real = std::f32::consts::PI;
}
