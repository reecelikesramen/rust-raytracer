use super::*;

#[derive(Debug)]
pub struct DiffuseLight {
    emission: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emission: Arc<dyn Texture>) -> Self {
        Self { emission }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, uv: (Real, Real), point: &P3) -> Color {
        self.emission.color(uv, point)
    }
}
