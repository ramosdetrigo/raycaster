use crate::{Intersection, Ray};
mod cilinder;
mod cone;
mod plane;
mod sphere;

pub use cilinder::Cilinder;
pub use cone::Cone;
pub use plane::Plane;
pub use sphere::Sphere;

pub trait Object: Sync {
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
}
