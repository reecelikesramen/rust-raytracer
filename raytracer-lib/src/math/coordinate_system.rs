use crate::{prelude::*, vec3};

#[derive(Debug)]
pub struct CoordinateSystem {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub position: Vec3,
}

impl CoordinateSystem {
    pub fn new(position: Vec3, view_direction: &Vec3) -> Self {
        let w = -view_direction.normalize();
        let mut temp_up = vec3!(0.0, 1.0, 0.0);
        let tdotw = temp_up.dot(&w);
        if tdotw.abs() > 0.999 {
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

        // let (u, v) = create_coordinate_system(&w);
        Self {
            u: u.normalize(),
            v: v.normalize(),
            w,
            position,
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

// Helper function to create a coordinate system from a normal
pub fn create_coordinate_system(normal: &Vec3) -> (Vec3, Vec3) {
    let tangent = if normal.x.abs() > 0.99 {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let bitangent = normal.cross(&tangent).normalize();
    let tangent = bitangent.cross(normal).normalize();
    (tangent, bitangent)
}
