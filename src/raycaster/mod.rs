pub mod shapes;
pub mod lights;

mod camera;
mod intersection;
mod material;
mod ray;
mod scene;

pub use camera::Camera;
pub use intersection::Intersection;
pub use material::Material;
pub use ray::Ray;
pub use scene::Scene;
