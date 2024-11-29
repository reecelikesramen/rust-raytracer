use crate::{
    geometry::Shape,
    hit_record::{HitData, HitRecord},
};
use std::sync::Arc;

use super::BBox;

// Axis enum for splitting
#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn next(&self) -> Self {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::Z,
            Axis::Z => Axis::X,
        }
    }
}

#[derive(Debug)]
pub struct BVHNode {
    bbox: BBox,
    left: Option<Box<BVHNode>>,
    right: Option<Box<BVHNode>>,
    shapes: Vec<Arc<dyn Shape>>,
}

impl BVHNode {
    // Maximum shapes in a leaf node
    const MAX_SHAPES: usize = 4;

    fn new(mut shapes: Vec<Arc<dyn Shape>>, axis: Axis) -> Self {
        // If we have few enough shapes, make a leaf node
        if shapes.len() <= Self::MAX_SHAPES {
            // Calculate bounding box for all shapes
            let bbox = shapes
                .iter()
                .fold(None, |acc, shape| {
                    let shape_bbox = shape.get_bbox().clone();
                    match acc {
                        None => Some(shape_bbox),
                        Some(bbox) => Some(BBox::combine(&bbox, &shape_bbox)),
                    }
                })
                .unwrap();

            return Self {
                bbox,
                left: None,
                right: None,
                shapes,
            };
        }

        // Sort shapes based on their centroids along the current axis
        shapes.sort_by(|a, b| {
            let a_centroid = a.get_centroid();
            let b_centroid = b.get_centroid();
            match axis {
                Axis::X => a_centroid.x.partial_cmp(&b_centroid.x).unwrap(),
                Axis::Y => a_centroid.y.partial_cmp(&b_centroid.y).unwrap(),
                Axis::Z => a_centroid.z.partial_cmp(&b_centroid.z).unwrap(),
            }
        });

        // Split shapes into two groups
        let mid = shapes.len() / 2;
        let right_shapes = shapes.split_off(mid);

        // Recursively build child nodes
        let left = Box::new(Self::new(shapes, axis.next()));
        let right = Box::new(Self::new(right_shapes, axis.next()));

        // Calculate bounding box for this node
        let bbox = BBox::combine(left.bbox(), right.bbox());

        Self {
            bbox,
            left: Some(left),
            right: Some(right),
            shapes: Vec::new(), // Internal nodes don't store shapes
        }
    }

    pub fn bbox(&self) -> &BBox {
        &self.bbox
    }

    pub fn closest_hit(&self, hit: &mut HitRecord) -> bool {
        // First check if ray intersects this node's bounding box
        if self.bbox.hit(&hit.ray, hit.t_min, hit.t).is_none() {
            return false;
        }

        let mut hit_anything = false;

        // If this is a leaf node, check all shapes
        if !self.shapes.is_empty() {
            for shape in &self.shapes {
                if shape.closest_hit(hit) {
                    hit_anything = true;
                }
            }
            return hit_anything;
        }

        // Otherwise, recurse into children
        if let Some(left) = &self.left {
            if left.closest_hit(hit) {
                hit_anything = true;
            }
        }

        if let Some(right) = &self.right {
            if right.closest_hit(hit) {
                hit_anything = true;
            }
        }

        hit_anything
    }
}

#[derive(Debug)]
pub struct BVH {
    root: BVHNode,
}

impl BVH {
    pub fn new(shapes: Vec<Arc<dyn Shape>>) -> Self {
        Self {
            root: BVHNode::new(shapes.clone(), Axis::X),
        }
    }

    pub fn closest_hit(&self, hit: &mut HitRecord) -> bool {
        self.root.closest_hit(hit)
    }

    pub fn get_closest_hit_data(&self, hit: &mut HitRecord) -> Option<HitData> {
        self.closest_hit(hit);
        hit.hit_data.take()
    }

    pub fn get_bbox(&self) -> &BBox {
        &self.root.bbox
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::geometry::{Sphere, Triangle};
//     use crate::shader::NormalShader;

//     #[test]
//     fn test_bvh_construction() {
//         let shader = Arc::new(NormalShader::new());
//         let shapes: Vec<Arc<dyn Shape>> = vec![
//             Arc::new(Sphere::new(
//                 V3::new(0.0, 0.0, -5.0),
//                 1.0,
//                 shader.clone(),
//                 "sphere1",
//             )),
//             Arc::new(Sphere::new(
//                 V3::new(2.0, 0.0, -5.0),
//                 1.0,
//                 shader.clone(),
//                 "sphere2",
//             )),
//         ];

//         let bvh = BVH::new(shapes);
//         assert!(bvh.root.bbox.hit(
//             &Ray {
//                 origin: V3::new(0.0, 0.0, 0.0),
//                 direction: V3::new(0.0, 0.0, -1.0),
//             },
//             0.0,
//             f64::INFINITY,
//         ).is_some());
//     }
// }
