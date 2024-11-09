use crate::prelude::*;
use vec3;
use approx::relative_eq;

struct CoordinateSystem {
	u: Vec3,
	v: Vec3,
	w: Vec3,
	position: Vec3
}

impl CoordinateSystem {
	fn new(position: Vec3, view_direction: Vec3) -> Self {
		let w = -view_direction.normalize();

		let mut temp_up = vec3!(0.0, 1.0, 0.0);
		let tdotw = temp_up.dot(&w);

		if relative_eq!(tdotw.abs(), 1.0) {
			temp_up = w.clone();

			let x = temp_up.x.abs();
			let y = temp_up.y.abs();
			let z = temp_up.z.abs();

			if x <= y && x <= z {
				temp_up.x = 1.0;
			} else if y <= x {
				temp_up.y = 1.0;
			} else {
				temp_up.z = 1.0;
			}
		}

		let u = temp_up.cross(&w);
		let v = w.cross(&u);

		#[cfg(debug_assertions)]
		{
			let _u = u.normalize();
			let _v = v.normalize();

			eprintln!("U: {}", _u);
			eprintln!("V: {}", _v);
			eprintln!("W: {}", w);
			eprintln!("View dir: {}", view_direction);
			println!("Temp up: {}", temp_up);

			assert!((w.dot(&_u) - 0.0).abs() < f64::EPSILON);
			assert!((w.dot(&_v) - 0.0).abs() < f64::EPSILON);
			assert!((_u.dot(&_v) - 0.0).abs() < f64::EPSILON);
			assert!((_u.magnitude() - 1.0).abs() < f64::EPSILON);
			assert!((_v.magnitude() - 1.0).abs() < f64::EPSILON);
			assert!((w.magnitude() - 1.0).abs() < f64::EPSILON);
		}

		Self {
			u: u.normalize(),
			v: v.normalize(),
			w,
			position
		}
	}

	fn to_local(&self, global: Vec3) -> Vec3 {
		let temp = global - self.position;

		vec3!(self.u.dot(&temp), self.v.dot(&temp), self.w.dot(&temp))
	}

	fn to_global(&self, local: Vec3) -> Vec3 {
		vec3!(self.u.dot(&local), self.v.dot(&local), self.w.dot(&local)) + self.position
	}
}