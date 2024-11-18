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
        let (u, v) = create_coordinate_system(&w);
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
