use crate::{Intersection, Scene};
use glam::DVec3;

mod point;

pub use point::Point;

/// """Classe""" 'luz'
pub trait Light: Sync {
    /// Calcula a intensidade/cor da luz em um determinado ponto de interseção numa cena
    /// `v: vetor unitário do ponto de interseção em direção ao observador`
    fn color_at(&self, intersection: &Intersection, v: DVec3, scene: &Scene) -> DVec3;
}