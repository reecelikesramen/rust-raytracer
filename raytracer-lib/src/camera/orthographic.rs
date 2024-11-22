use super::*;
use crate::math::Ray;
use crate::V3;

#[derive(Debug)]
pub struct OrthographicCamera {
    base: CameraBase,
}

impl OrthographicCamera {
    pub fn new(position: P3, view_direction: &V3, aspect_ratio: Real) -> Self {
        let image_plane_width = DEFAULT_IMAGE_PLANE_WIDTH;
        let image_plane_height = image_plane_width / aspect_ratio;
        Self {
            base: CameraBase::new(
                position,
                view_direction,
                image_plane_width,
                image_plane_height,
            ),
        }
    }
}

impl Camera for OrthographicCamera {
    fn generate_ray(&self, i: u32, j: u32, di: Real, dj: Real) -> Ray {
        let (u, v) = self.base.get_uv(i, j, di, dj);
        let origin = self.base.basis.position + V3::new(u, v, 0.0);
        Ray {
            origin,
            direction: V3::new(0.0, 0.0, -1.0),
        }
    }

    fn camera_base(&self) -> &CameraBase {
        &self.base
    }
    fn camera_base_mut(&mut self) -> &mut CameraBase {
        &mut self.base
    }
}
