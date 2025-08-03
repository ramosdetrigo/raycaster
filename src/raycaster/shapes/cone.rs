use crate::{Intersection, Material, Ray, shapes::Shape};
use glam::DVec3;

/// Cone definido pelo centro de sua base, direção, altura e raio da base
pub struct Cone {
    /// Centro da base
    pub cb: DVec3,
    /// Direção do eixo do cone
    pub dc: DVec3,
    /// Altura do cone
    pub height: f64,
    /// Raio da base do cone
    pub radius: f64,
    /// Material do cone
    pub material: Material,
    /// Booleano que indica se o cone tem base ou não
    pub has_base: bool,
}

impl Cone {
    /// Construtor do cone
    pub fn new(
        cb: DVec3,
        dc: DVec3,
        height: f64,
        radius: f64,
        has_base: bool,
        material: Material,
    ) -> Cone {
        Cone {
            cb,
            dc,
            height,
            radius,
            has_base,
            material,
        }
    }

    /// Retorna a interseção mais próxima entre a superfície cônica
    /// e a base do cone (`None` se não há interseção).
    pub fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        // Testa interseção com a
        let surface_intersection = self.surface_intersects(ray);
        let base_intersection = if self.has_base {
            self.base_intersects(ray)
        } else {
            None
        };

        // Pega a interseção mais próxima dentre as 2 calculadas
        [surface_intersection, base_intersection]
            .into_iter()
            .flatten() // Pega só as que não forem "None"
            .min_by(|intersection1, intersection2| intersection1.t.total_cmp(&intersection2.t))
    }

    /// Retorna a interseção de menor t positivo dum raio com a superfície do cone
    /// (`None` se não há interseção).
    fn surface_intersects(&self, ray: &Ray) -> Option<Intersection> {
        // Resolve a equação de segundo grau em t
        // para a semelhança de triângulos |MCbP| / |hdc - QCbP| = r / h
        let w = ray.p0 - self.cb;

        let mdr = ray.dr.reject_from_normalized(self.dc);
        let mw = w.reject_from_normalized(self.dc);
        let qdr = ray.dr.project_onto_normalized(self.dc);
        let qw = w.project_onto_normalized(self.dc);
        let hdc = self.height * self.dc;

        let h2 = self.height * self.height;
        let r2 = self.radius * self.radius;

        let a = h2 * mdr.length_squared() - r2 * qdr.length_squared();
        let b = 2.0 * (h2 * mdr.dot(mw) + r2 * qdr.dot(hdc - qw));
        let c = h2 * mw.length_squared() - r2 * (qw - hdc).length_squared();

        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            return None;
        }

        // Pega a interseção válida de menor t positivo
        let t1 = (-b - delta.sqrt()) / (2.0 * a);
        let t2 = (-b + delta.sqrt()) / (2.0 * a);
        // A gente tem que testar as 2 interseções antes de pegar o t mínimo,
        // por conta da peculiaridade da superfície cônica se estender ao infinito
        // pra cima, podendo causar problemas de não renderizar o cone por conta da
        // "interseção fantasma" com a superfície de cima etc.
        [t1, t2]
            .into_iter()
            .filter_map(|t| {
                // Pega só as interseções de T positivo
                if t < 0.0 {
                    return None;
                }
                // Testa se o ponto de interseção está entre a base e o topo do cone
                let p = ray.at(t);
                let intersection_height = (p - self.cb).dot(self.dc);
                if intersection_height < 0.0 || intersection_height > self.height {
                    return None;
                }
                // Normal = Vetor M_pv*dc normalizado
                let vertice = self.cb + self.dc * self.height;
                let pv = (p - vertice).normalize();
                let n = (self.dc).reject_from_normalized(pv).normalize();
                Some(Intersection {
                    t,
                    p,
                    normal: n,
                    material: self.material,
                    object: self,
                })
            })
            // Pega a interseção com menor t
            .min_by(|intersection1, intersection2| intersection1.t.total_cmp(&intersection2.t))
    }

    /// Teste de interseção raio-plano com a base do cone
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
}

impl Shape for Cone {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        self.intersects(ray)
    }
}
