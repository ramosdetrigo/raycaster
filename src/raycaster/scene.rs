use crate::Camera;
use glam::DVec3;

pub struct Scene {
    cameras: Vec<Camera>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { cameras: vec![] }
    }

    pub fn new_camera(&mut self) -> &Camera {
        self.cameras.push(Camera::new(DVec3::ZERO, 16.0, 9.0, 8.0));
        self.cameras.last().unwrap()
    }
}
