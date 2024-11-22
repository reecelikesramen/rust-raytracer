use na::Vector2;
use rand::Rng;

use super::*;
use crate::math::Ray;

#[derive(Debug)]
pub struct PerspectiveCamera {
    base: CameraBase,
    defocus_angle: Real,
    focus_distance: Real,
    defocus_disk_u: V3,
    defocus_disk_v: V3,
}

impl PerspectiveCamera {
    pub fn new(
        position: P3,
        view_direction: &V3,
        aspect_ratio: Real,
        field_of_view: Real,
        focus_distance: Real,
        defocus_angle: Real,
    ) -> Self {
        // Calculate the viewport dimensions at the focus distance
        let h = (field_of_view / 2.).to_radians().tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * aspect_ratio;

        let base = CameraBase::new(
            position,
            view_direction,
            viewport_width,
            viewport_height,
        );

        let defocus_radius = focus_distance * (defocus_angle / 2.).to_radians().tan();
        let defocus_disk_u = base.basis.u * defocus_radius;
        let defocus_disk_v = base.basis.v * defocus_radius;

        Self {
            base,
            defocus_angle,
            focus_distance,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn old(position: P3, view_direction: &V3, aspect_ratio: Real, focal_length: Real) -> Self {
        let image_plane_height = 2.;
        let image_plane_width = image_plane_height * aspect_ratio;
        Self {
            base: CameraBase::new(
                position,
                view_direction,
                image_plane_width,
                image_plane_height,
            ),
            defocus_angle: 0.0,
            focus_distance: focal_length,
            defocus_disk_u: V3::zeros(),
            defocus_disk_v: V3::zeros(),
        }
    }

    fn defocus_disk_sample(&self) -> P3 {
        let (u, v) = random_in_unit_disk(&mut rand::thread_rng());
        self.base.basis.position + self.defocus_disk_u * u + self.defocus_disk_v * v
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, i: u32, j: u32, di: Real, dj: Real) -> Ray {
        let (u, v) = self.base.get_uv(i, j, di, dj);
        
        // Calculate the point on the viewport
        let viewport_point = self.base.basis.u * u + self.base.basis.v * v;
        
        // Get ray origin (either camera center or point on lens)
        let origin = if self.defocus_angle <= 0.0 {
            self.base.basis.position
        } else {
            self.defocus_disk_sample()
        };

        // Calculate the target point at the focus distance
        let target = self.base.basis.position + viewport_point - self.base.basis.w * self.focus_distance;
        
        // Direction is from origin through target
        let direction = (target - origin).normalize();

        Ray { origin, direction }
    }

    fn camera_base(&self) -> &CameraBase {
        &self.base
    }
    fn camera_base_mut(&mut self) -> &mut CameraBase {
        &mut self.base
    }
}

fn random_in_unit_disk(rand: &mut rand::rngs::ThreadRng) -> (Real, Real) {
    loop {
        let p = Vector2::new(rand.gen::<Real>(), rand.gen::<Real>());
        if p.magnitude_squared() < 1.0 {
            return (p.x, p.y);
        }
    }
}
