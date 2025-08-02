use crate::{Intersection, Ray};
mod sphere;

pub use sphere::Sphere;

pub trait Object: Sync {
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
}

