use crate::raycaster::{Intersection, Material, Ray};
use nalgebra::{UnitVector3, Vector3};

pub struct Sphere {
    pub pos: Vector3<f64>,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    /// Retorna a interseção de um raio com uma esfera (None se não há interseção)
    pub fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        // resolvemos a equação do segundo grau |R(t) - C| = r
        // t²*dr•dr + 2t*dr•v + v•v - r² = 0
        // v = p0 - ce (centro da esfera)

        // origin-to-center
        let oc = ray.p0 - self.pos;

        let a = ray.dr.magnitude_squared();
        let b = 2.0 * ray.dr.dot(&oc);
        let c = oc.magnitude_squared() - self.radius * self.radius;
        let delta = b * b - 4.0 * a * c;

        if delta >= 0.0 {
            let t1 = (-b + delta.sqrt()) / (2.0 * a);
            let t2 = (-b - delta.sqrt()) / (2.0 * a);

            // Pega o t > 0 mais próximo e constrói um struct da interseção nesse t
            // (None se não há t > 0)
            [t1, t2]
                .into_iter()
                .filter(|&t| t > 0.0)
                .min_by(|t1, t2| t1.total_cmp(t2))
                .map(|t| {
                    let p = ray.at(t);
                    let n = UnitVector3::new_normalize(p - self.pos);
                    Intersection {
                        t: t,
                        p: p,
                        normal: n,
                        material: self.material,
                    }
                })
        } else {
            None
        }
    }
}
