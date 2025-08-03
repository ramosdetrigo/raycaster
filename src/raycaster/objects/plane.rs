use glam::DVec3;

use crate::{Intersection, Material, Ray, objects::Object};

pub struct Plane {
    pub pc: DVec3,
    pub normal: DVec3,
    pub material: Material,
}

impl Plane {
    pub fn new(pc: DVec3, normal: DVec3, material: Material) -> Plane {
        Plane {
            pc,
            normal,
            material,
        }
    }

    pub fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let bottom = ray.dr.dot(self.normal);
        if bottom.abs() < 1e-8 {
            return None;
        }
        let t = -(ray.p0 - self.pc).dot(self.normal) / bottom;
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
