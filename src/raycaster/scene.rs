use glam::DVec3;

use crate::{lights::Light, shapes::Shape};

/// Uma cena que guarda nossos objetos e luzes
pub struct Scene {
    /// Objetos na cena
    pub objects: Vec<Box<dyn Shape>>,
    /// Luzes na cena
    pub lights: Vec<Box<dyn Light>>,
    /// Luz ambiente da cena
    pub ambient_light: DVec3
}
