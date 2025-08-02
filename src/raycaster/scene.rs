use glam::DVec3;

use crate::{lights::Light, objects::Object};

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Box<dyn Light>>,
    pub ambient_light: DVec3
}
