use na::Matrix4;

use crate::{math::Ray, prelude::*, V3};

#[derive(Debug, Clone)]
pub struct BBox {
    pub min: P3,
    pub max: P3,
    pub centroid: P3,
    pub extent: V3,
}

impl BBox {
    pub fn new(min: P3, max: P3) -> BBox {
        BBox {
            min,
            max,
            centroid: na::center(&min, &max),
            extent: max - min,
        }
    }

    pub fn combine(b1: &BBox, b2: &BBox) -> BBox {
        BBox::new(
            P3::new(
                b1.min.x.min(b2.min.x),
                b1.min.y.min(b2.min.y),
                b1.min.z.min(b2.min.z),
            ),
            P3::new(
                b1.max.x.max(b2.max.x),
                b1.max.y.max(b2.max.y),
                b1.max.z.max(b2.max.z),
            ),
        )
    }

    pub fn transform(&self, transform: &Matrix4<Real>) -> BBox {
        // Apply the transformation to the 8 corners of the AABB
        let corners = [
            self.min,
            P3::new(self.max.x, self.min.y, self.min.z),
            P3::new(self.min.x, self.max.y, self.min.z),
            P3::new(self.max.x, self.max.y, self.min.z),
            P3::new(self.min.x, self.min.y, self.max.z),
            P3::new(self.max.x, self.min.y, self.max.z),
            P3::new(self.min.x, self.max.y, self.max.z),
            self.max,
        ];

        // Transform all corners
        let mut transformed_corners = Vec::new();
        for corner in corners.iter() {
            // Apply the homogeneous transformation (multiply matrix by point)
            let transformed = transform.transform_point(corner);
            transformed_corners.push(transformed.xyz());
        }

        // Calculate the new AABB's min and max
        let mut min = transformed_corners[0];
        let mut max = transformed_corners[0];

        for corner in transformed_corners.iter() {
            min.x = min.x.min(corner.x);
            min.y = min.y.min(corner.y);
            min.z = min.z.min(corner.z);

            max.x = max.x.max(corner.x);
            max.y = max.y.max(corner.y);
            max.z = max.z.max(corner.z);
        }

        BBox::new(min, max)
    }

    pub fn hit(&self, ray: &Ray, mut tmin: Real, mut tmax: Real) -> Option<Real> {
        let r_to_min = self.min - ray.origin;
        let r_to_max = self.max - ray.origin;
        let dir = ray.direction;

        // Handle x-axis intersection
        let mut tmin_x = r_to_min.x / dir.x;
        let mut tmax_x = r_to_max.x / dir.x;
        if 1.0 / dir.x < 0.0 {
            std::mem::swap(&mut tmin_x, &mut tmax_x);
        }

        // Early exit if no x-axis overlap
        if tmin_x >= tmax_x || tmin_x >= tmax || tmax_x <= tmin {
            return None;
        }

        tmax = tmax.min(tmax_x);
        tmin = tmin.max(tmin_x);

        // Handle y-axis intersection
        let mut tmin_y = r_to_min.y / dir.y;
        let mut tmax_y = r_to_max.y / dir.y;
        if 1.0 / dir.y < 0.0 {
            std::mem::swap(&mut tmin_y, &mut tmax_y);
        }

        // Early exit if no y-axis overlap
        if tmin_y >= tmax_y || tmin_y >= tmax || tmax_y <= tmin {
            return None;
        }

        tmax = tmax.min(tmax_y);
        tmin = tmin.max(tmin_y);

        // Handle z-axis intersection
        let mut tmin_z = r_to_min.z / dir.z;
        let mut tmax_z = r_to_max.z / dir.z;
        if 1.0 / dir.z < 0.0 {
            std::mem::swap(&mut tmin_z, &mut tmax_z);
        }

        // Final check for z-axis overlap
        if tmin_z >= tmax_z || tmin_z >= tmax || tmax_z <= tmin {
            return None;
        }

        // Return the intersection point
        Some(tmin_z.max(tmin))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbox_ray_intersections() {
        // Create a default BBox
        let b1 = BBox::new(P3::new(-0.25, -0.25, -2.25), P3::new(0.25, 0.25, -1.75));

        // Create test rays equivalent to C++ test
        let r1 = Ray {
            origin: P3::new(0.0, 0.0, 0.0),
            direction: V3::new(0.0, 0.0, -1.0),
        };

        let r2 = Ray {
            origin: P3::new(0.0, 0.0, 0.0),
            direction: V3::new(0.0, 0.0, 1.0),
        };

        let r3 = Ray {
            origin: P3::new(1.25, 1.25, 0.25),
            direction: V3::new(-1.0, -1.0, -2.0),
        };

        let r4 = Ray {
            origin: P3::new(0.0, 0.0, 0.0),
            direction: V3::new(-2.0, -2.0, -1.0),
        };

        // Create larger bounding box for r5 test
        let b2 = BBox::new(P3::new(-10.0, -300.0, -8.0), P3::new(302.0, 300.0, 600.0));

        let r5 = Ray {
            origin: P3::new(80.0, -100.0, 300.0),
            direction: V3::new(0.1871, 0.6359, -0.7488),
        };

        // Test ray intersections
        assert!(b1.hit(&r1, 1.0, f64::INFINITY).is_some());
        assert!(b1.hit(&r2, 1.0, f64::INFINITY).is_none());
        assert!(b1.hit(&r3, 1.0, f64::INFINITY).is_some());
        assert!(b1.hit(&r4, 1.0, f64::INFINITY).is_none());
        assert!(b2.hit(&r5, 1.0, f64::INFINITY).is_some());
    }
}
