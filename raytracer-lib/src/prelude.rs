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