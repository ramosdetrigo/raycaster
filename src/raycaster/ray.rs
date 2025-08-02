use nalgebra::{UnitVector3, Vector3};

/// Raio definido pela função p = p0 + dr*t
pub struct Ray {
    pub p0: Vector3<f64>,
    pub dr: UnitVector3<f64>,
}

impl Ray {
    /// Retorna ponto t do raio (p0 + dr*t)
    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.p0 + self.dr.scale(t)
    }
}
