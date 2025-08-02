use crate::raycaster::Material;
use nalgebra::{UnitVector3, Vector3};

/// Representação de uma interseção entre um raio e um objeto
pub struct Intersection {
    /// Posição t do raio R(t)
    pub t: f64,
    /// Ponto P da interseção
    pub p: Vector3<f64>,
    /// Vetor normal da interseção
    pub normal: UnitVector3<f64>,
    /// Material no ponto de interseção
    pub material: Material,
}
