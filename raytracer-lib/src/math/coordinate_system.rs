use crate::{prelude::*, V3};

#[derive(Debug)]
pub struct CoordinateSystem {
    pub u: V3,
    pub v: V3,
    pub w: V3,
    pub position: P3,
}

impl CoordinateSystem {
    pub fn new(position: P3, view_direction: &V3) -> Self {
        let w = -view_direction.normalize();
        let mut temp_up = V3::new(0.0, 1.0, 0.0);
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
}

// Helper function to create a coordinate system from a normal
pub fn create_coordinate_system(normal: &V3) -> (V3, V3) {
    let tangent = if normal.x.abs() > 0.99 {
        V3::new(0.0, 1.0, 0.0)
    } else {
        V3::new(1.0, 0.0, 0.0)
    };
    let bitangent = normal.cross(&tangent).normalize();
    let tangent = bitangent.cross(normal).normalize();
    (tangent, bitangent)
}
