use std::sync::Arc;

use crate::{prelude::*, shader::Shader, vec3};

use super::{bbox::BBox, Shape};

#[derive(Debug)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    normal: Vec3,
    bbox: BBox,
    shader: Arc<dyn Shader>,
    name: &'static str,
}

impl Triangle {
    fn new(a: Vec3, b: Vec3, c: Vec3, shader: Arc<dyn Shader>, name: &'static str) -> Self {
        let normal = (b - a).cross(&(c - a)).normalize();
        let min = vec3!(
            a.x.min(b.x).min(c.x),
            a.y.min(b.y).min(c.y),
            a.z.min(b.z).min(c.z)
        );
        let max = vec3!(
            a.x.max(b.x).max(c.x),
            a.y.max(b.y).max(c.y),
            a.z.max(b.z).max(c.z)
        );
        Self {
            a,
            b,
            c,
            normal,
            bbox: BBox::new(min, max),
            shader,
            name,
        }
    }
}

impl Shape for Triangle {
    fn get_type(&self) -> super::ShapeType {
        super::ShapeType::Triangle
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn get_bbox(&self) -> &BBox {
        &self.bbox
    }

    fn get_centroid(&self) -> Vec3 {
        self.bbox.centroid
    }

    fn get_shader(&self) -> Arc<dyn Shader> {
        Arc::clone(&self.shader)
    }

    fn closest_hit<'hit>(&'hit self, hit: &mut crate::shader::Hit<'hit>) -> bool {
        todo!()
        /* original C++ implementation:
        // e = r.origin, d = r.direction
        // triangle with vertices a, b, c
        // e + td = a + B(b-a) + Y(b-c) for some intersectT > 0, B > 0, Y > 0, and B+Y
        // < 1
        // Following code is based directly off cramer's rule in the book
        // This is made to get a quick and dirty attempt at ray tracing a triangle
        // Code can be heavily optimized

        std::vector<double> matrixA(9), matrixBeta(9), matrixGamma(9), matrixT(9);

        matrixA = { // 1st row
                    m_a[0] - m_b[0], m_a[0] - m_c[0], r.direction()[0],
                    // 2nd row
                    m_a[1] - m_b[1], m_a[1] - m_c[1], r.direction()[1],
                    // 3rd row
                    m_a[2] - m_b[2], m_a[2] - m_c[2], r.direction()[2]
        };

        double detA =
          matrixA[0] * (matrixA[4] * matrixA[8] - matrixA[5] * matrixA[7]) +
          matrixA[3] * (matrixA[2] * matrixA[7] - matrixA[1] * matrixA[8]) +
          matrixA[6] * (matrixA[1] * matrixA[5] - matrixA[4] * matrixA[2]);

        matrixT = {
          // 1st row
          m_a[0] - m_b[0], m_a[0] - m_c[0], m_a[0] - r.origin()[0],
          // 2nd row
          m_a[1] - m_b[1], m_a[1] - m_c[1], m_a[1] - r.origin()[1],
          // 3rd row
          m_a[2] - m_b[2], m_a[2] - m_c[2], m_a[2] - r.origin()[2],
        };

        double detT =
          matrixT[0] * (matrixT[4] * matrixT[8] - matrixT[5] * matrixT[7]) +
          matrixT[3] * (matrixT[2] * matrixT[7] - matrixT[1] * matrixT[8]) +
          matrixT[6] * (matrixT[1] * matrixT[5] - matrixT[4] * matrixT[2]);

        double intersectT = detT / detA;

        if (intersectT < hit.tmin || intersectT > hit.tmax)
          return false;

        matrixGamma = { // 1st row
                        m_a[0] - m_b[0], m_a[0] - r.origin()[0], r.direction()[0],
                        // 2nd row
                        m_a[1] - m_b[1], m_a[1] - r.origin()[1], r.direction()[1],
                        // 3rd row
                        m_a[2] - m_b[2], m_a[2] - r.origin()[2], r.direction()[2]
        };

        double detGamma =
          matrixGamma[0] *
            (matrixGamma[4] * matrixGamma[8] - matrixGamma[5] * matrixGamma[7]) +
          matrixGamma[3] *
            (matrixGamma[2] * matrixGamma[7] - matrixGamma[1] * matrixGamma[8]) +
          matrixGamma[6] *
            (matrixGamma[1] * matrixGamma[5] - matrixGamma[4] * matrixGamma[2]);

        double gamma = detGamma / detA;

        if (gamma < 0 || gamma > 1)
          return false;

        matrixBeta = { // 1st row
                       m_a[0] - r.origin()[0], m_a[0] - m_c[0], r.direction()[0],
                       // 2nd row
                       m_a[1] - r.origin()[1], m_a[1] - m_c[1], r.direction()[1],
                       // 3rd row
                       m_a[2] - r.origin()[2], m_a[2] - m_c[2], r.direction()[2]
        };

        double detBeta =
          matrixBeta[0] *
            (matrixBeta[4] * matrixBeta[8] - matrixBeta[5] * matrixBeta[7]) +
          matrixBeta[3] *
            (matrixBeta[2] * matrixBeta[7] - matrixBeta[1] * matrixBeta[8]) +
          matrixBeta[6] *
            (matrixBeta[1] * matrixBeta[5] - matrixBeta[4] * matrixBeta[2]);

        double beta = detBeta / detA;

        if (beta < 0 || beta > 1 - gamma)
          return false;

        hit.ray = r;
        hit.shaderPtr = shaderPtr;
        hit.shape = this;
        hit.t = intersectT;
        hit.normal = normal();

        return true;
               */
    }
}
