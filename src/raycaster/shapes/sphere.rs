use crate::{
    shapes::Shape,
    raycaster::{Intersection, Material, Ray},
};
use glam::DVec3;

/// Esfera definida pelo seu centro e seu raio.
pub struct Sphere {
    /// Centro da esfera
    pub pos: DVec3,
    /// Raio da esfera
    pub radius: f64,
    /// Material da esfera
    pub material: Material,
}

impl Sphere {
    /// Construtor da esfera
    pub fn new(pos: DVec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            pos,
            radius,
            material,
        }
    }

    /// Retorna a interseção mais próxima de um raio com uma esfera (None se não há interseção)
    pub fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        // resolvemos a equação do segundo grau |R(t) - C| = r
        // t²*dr•dr + 2t*dr•v + v•v - r² = 0
        // v = p0 - ce (centro da esfera)

        // origin-to-center
        let oc = ray.p0 - self.pos;

        let a = ray.dr.length_squared();
        let b = 2.0 * ray.dr.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let delta = b * b - 4.0 * a * c;

        if delta < 0.0 {
            return None;
        }

        // Pega o t > 0 mais próximo e constrói um struct da interseção nesse t
        // (None se não há t > 0)
        let t1 = (-b + delta.sqrt()) / (2.0 * a);
        let t2 = (-b - delta.sqrt()) / (2.0 * a);
        [t1, t2]
            .into_iter()
            .filter(|&t| t > 0.0)
            .min_by(|t1, t2| t1.total_cmp(t2))
            .map(|t| {
                let p = ray.at(t);
                let n = (p - self.pos).normalize();
                Intersection {
                    t: t,
                    p: p,
                    normal: n,
                    material: self.material,
                    object: self,
                }
            })
    }
}

impl Shape for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        self.intersects(ray)
    }
}
