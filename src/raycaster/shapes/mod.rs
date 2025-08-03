use crate::{Intersection, Ray};
mod cilinder;
mod cone;
mod plane;
mod sphere;

pub use cilinder::Cilinder;
pub use cone::Cone;
pub use plane::Plane;
pub use sphere::Sphere;

/// """Classe""" 'objeto' com método que diz se um raio o intersecta ou não
pub trait Shape: Sync {
    /// Retorna a interseção de um raio com o objeto de t positivo mais próxima
    /// (`None` se não há colisão).
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
}
