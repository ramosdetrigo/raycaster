use glam::DVec3;

use crate::{Intersection, Material, Ray, objects::Object};

/// Cilindro definido pelo centro de sua base, direção, altura e raio
pub struct Cilinder {
    /// Centro da base do cilindro
    pub cb: DVec3,
    /// Direção do eixo do cilindro
    pub dc: DVec3,
    /// Altura do cilindro
    pub height: f64,
    /// Raio do cilindro
    pub radius: f64,
    /// Material do cilindro
    pub material: Material,
    /// Booleano que indica se o cilindro tem base ou não
    pub has_base: bool,
    /// Booleano que indica se o cilindro tem topo ou não
    pub has_top: bool,
}

impl Cilinder {
    /// Construtor do cilindro
    pub fn new(
        cb: DVec3,
        dc: DVec3,
        height: f64,
        radius: f64,
        has_base: bool,
        has_top: bool,
        material: Material,
    ) -> Cilinder {
        Cilinder {
            cb,
            dc,
            height,
            radius,
            has_base,
            has_top,
            material,
        }
    }

    /// Retorna a interseção mais próxima entre a superfície cilíndrica,
    /// a base e o topo do cilindro (`None` se não há interseção).
    pub fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        // Testa interseção com a superfície, a base e o topo do cilindro
        let surface_intersection = self.surface_intersects(ray);
        let base_intersection = if self.has_base {
            self.base_intersects(ray)
        } else {
            None
        };
        let top_intersection = if self.has_top {
            self.top_intersects(ray)
        } else {
            None
        };

        // Pega a interseção mais próxima dentre as 3 calculadas
        [surface_intersection, base_intersection, top_intersection]
            .into_iter()
            .flatten() // Pega só as que não forem "None"
            .min_by(|intersection1, intersection2| intersection1.t.total_cmp(&intersection2.t))
    }

    /// Retorna a interseção de menor t positivo dum raio com a superfície do cilindro
    /// (`None` se não há interseção)
    fn surface_intersects(&self, ray: &Ray) -> Option<Intersection> {
        // Resolve a equação do segudo grau para |M*CbP| = r
        // (o ponto P de interseção do raio deve estar a uma distância
        // r do eixo do cilindro)
        let w = ray.p0 - self.cb;

        let mdr = ray.dr.reject_from_normalized(self.dc);
        let mw = w.reject_from_normalized(self.dc);

        let a = mdr.length_squared();
        let b = 2.0 * mdr.dot(mw);
        let c = mw.length_squared() - self.radius * self.radius;

        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            return None;
        }

        // Pega a interseção válida de menor t positivo
        let t1 = (-b - delta.sqrt()) / (2.0 * a);
        let t2 = (-b + delta.sqrt()) / (2.0 * a);
        [t1, t2]
            .into_iter()
            .filter(|t| *t > 0.0) // Filtra só os T's positivos
            .min_by(|t1, t2| t1.total_cmp(t2)) // Pega o menor t positivo
            .and_then(|t| {
                let p = ray.at(t);
                // Testa se o ponto de interseção está entre a base e o topo do cilindro
                let intersection_height = (p - self.cb).dot(self.dc);
                if intersection_height < 0.0 || intersection_height > self.height {
                    return None;
                }
                // Normal = Vetor M*CbP normalizado
                let n = (p - self.cb).reject_from_normalized(self.dc).normalize();
                Some(Intersection {
                    t,
                    p,
                    normal: n,
                    material: self.material,
                    object: self,
                })
            })
    }

    /// Teste de interseção raio-plano com a base do cilindro
    fn base_intersects(&self, ray: &Ray) -> Option<Intersection> {
        let bottom = ray.dr.dot(-self.dc);
        if bottom.abs() < 1e-8 {
            return None;
        }
        let t = -(ray.p0 - self.cb).dot(-self.dc) / bottom;
        // O ponto de interseção deve estar a uma distância de no máximo r do centro da base
        let p = ray.at(t);
        if t < 0.0 || (p - self.cb).length_squared() > self.radius * self.radius {
            return None;
        }
        Some(Intersection {
            t: t,
            p: p,
            normal: -self.dc,
            material: self.material,
            object: self,
        })
    }

    /// Teste de interseção raio-plano com o topo do cilindro
    fn top_intersects(&self, ray: &Ray) -> Option<Intersection> {
        let bottom = ray.dr.dot(self.dc);
        if bottom.abs() < 1e-8 {
            return None;
        }
        let ct = self.cb + self.dc * self.height;
        let t = -(ray.p0 - ct).dot(self.dc) / bottom;
        let p = ray.at(t);
        // O ponto de interseção deve estar a uma distância de no máximo r do centro do topo
        if t < 0.0 || (p - ct).length_squared() > self.radius * self.radius {
            return None;
        }
        Some(Intersection {
            t: t,
            p: p,
            normal: self.dc,
            material: self.material,
            object: self,
        })
    }
}

impl Object for Cilinder {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        self.intersects(ray)
    }
}
