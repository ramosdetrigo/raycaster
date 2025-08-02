use crate::raycaster::Material;
use crate::raycaster::objects::Object;
use glam::DVec3;

/// Representação de uma interseção entre um raio e um objeto
pub struct Intersection<'a> {
    /// Posição t do raio R(t)
    pub t: f64,
    /// Ponto P da interseção
    pub p: DVec3,
    /// Vetor normal da interseção
    pub normal: DVec3,
    /// Material no ponto de interseção
    pub material: Material,
    /// Objeto colidido
    pub object: &'a dyn Object
}
