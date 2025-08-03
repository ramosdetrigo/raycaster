use glam::DVec3;

use crate::{Intersection, Material, Ray, objects::Object};

pub struct Cilinder {
    pub cb: DVec3,
    pub dc: DVec3,
    pub height: f64,
    pub radius: f64,
    pub material: Material,
    pub has_base: bool,
    pub has_top: bool,
}

impl Cilinder {
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

    pub fn intersects(&self, ray: &Ray) -> Option<Intersection> {
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

        [surface_intersection, base_intersection, top_intersection]
            .into_iter()
            .flatten()
            .min_by(|intersection1, intersection2| intersection1.t.total_cmp(&intersection2.t))
    }

    fn surface_intersects(&self, ray: &Ray) -> Option<Intersection> {
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

    fn top_intersects(&self, ray: &Ray) -> Option<Intersection> {
        let bottom = ray.dr.dot(self.dc);
        if bottom.abs() < 1e-8 {
            return None;
        }
        let ct = self.cb + self.dc * self.height;
        let t = -(ray.p0 - ct).dot(self.dc) / bottom;
        let p = ray.at(t);
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
