use crate::{Intersection, Material, Ray, objects::Object};
use glam::DVec3;

pub struct Cone {
    pub cb: DVec3,
    pub dc: DVec3,
    pub height: f64,
    pub radius: f64,
    pub material: Material,
    pub has_base: bool,
}

impl Cone {
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

    pub fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let surface_intersection = self.surface_intersects(ray);
        let base_intersection = if self.has_base {
            self.base_intersects(ray)
        } else {
            None
        };

        [surface_intersection, base_intersection]
            .into_iter()
            .flatten()
            .min_by(|intersection1, intersection2| intersection1.t.total_cmp(&intersection2.t))
    }

    fn surface_intersects(&self, ray: &Ray) -> Option<Intersection> {
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

        let t1 = (-b - delta.sqrt()) / (2.0 * a);
        let t2 = (-b + delta.sqrt()) / (2.0 * a);
        [t1, t2]
            .into_iter()
            .filter(|t| *t > 0.0)
            .min_by(|t1, t2| t1.total_cmp(t2))
            .and_then(|t| {
                let p = ray.at(t);
                let intersection_height = (p - self.cb).dot(self.dc);
                if intersection_height < 0.0 || intersection_height > self.height {
                    return None;
                }
                let vertice = self.cb + self.dc*self.height;
                let n = (p - vertice).reject_from_normalized(self.dc).normalize();
                Some(Intersection {
                    t,
                    p,
                    normal: n,
                    material: self.material,
                    object: self,
                })
            })
    }

    fn base_intersects(&self, ray: &Ray) -> Option<Intersection> {
        let bottom = ray.dr.dot(-self.dc);
        if bottom.abs() < 1e-8 {
            return None;
        }
        let t = -(ray.p0 - self.cb).dot(-self.dc) / bottom;
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

impl Object for Cone {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        self.intersects(ray)
    }
}
