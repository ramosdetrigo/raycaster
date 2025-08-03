use glam::DVec3;

/// Raio definido pela função p = p0 + dr*t
pub struct Ray {
    /// Ponto 0 do raio
    pub p0: DVec3,
    /// Direção do raio
    pub dr: DVec3,
}

impl Ray {
    #[inline]
    pub fn new(p0: DVec3, dr: DVec3) -> Ray {
        Ray { p0, dr }
    }

    /// Retorna ponto t do raio (p0 + dr*t)
    pub fn at(&self, t: f64) -> DVec3 {
        self.p0 + self.dr * t
    }
}
