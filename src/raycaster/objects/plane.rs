use glam::DVec3;

use crate::{Intersection, Material, Ray, objects::Object};

/// Plano definido por um ponto conhecido da superfície e sua normal
pub struct Plane {
    /// Ponto conhecido da superfície do plano
    pub pc: DVec3,
    /// Vetor normal ao plano
    pub normal: DVec3,
    /// Material do plano
    pub material: Material,
}

impl Plane {
    /// Construtor do plano
    pub fn new(pc: DVec3, normal: DVec3, material: Material) -> Plane {
        Plane {
            pc,
            normal,
            material,
        }
    }

    /// Retorna os dados da interseção de um raio com o plano (None se não há interseção).
    pub fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        // Resolve a equação em t (P - Pc) • n = 0
        // --> t = -(p0-pc)•n/dr•n
        let bottom = ray.dr.dot(self.normal);
        // Se o termo de baixo é igual a 0, o raio é paralelo ao plano.
        if bottom.abs() < 1e-8 {
            return None;
        }
        let t = -(ray.p0 - self.pc).dot(self.normal) / bottom;
        // Não retorna interseções atrás do p0 do raio
        if t < 0.0 {
            return None;
        }
        Some(Intersection {
            t: t,
            p: ray.at(t),
            normal: self.normal,
            material: self.material,
            object: self,
        })
    }
}

impl Object for Plane {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        self.intersects(ray)
    }
}
