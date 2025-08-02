use glam::{DMat3, DVec3};
use raylib::texture::Image;

pub struct Camera {
    pub p0: DVec3,
    pub frame_width: f64,
    pub frame_height: f64,
    pub frame_distance: f64,
    pub coord_system: DMat3,
}

impl Camera {
    #[inline]
    #[must_use]
    pub fn new(p0: DVec3, frame_width: f64, frame_height: f64, frame_distance: f64) -> Camera {
        Camera {
            p0,
            frame_width,
            frame_height,
            frame_distance,
            coord_system: DMat3::IDENTITY,
        }
    }

    /// Ponto correspondente ao centro do frame
    fn frame_center(&self) -> DVec3 {
        -self.frame_distance * self.coord_system.z_axis
    }

    /// Ponto correspondente à quina superior-esquerda do frame
    fn frame_00(&self) -> DVec3 {
        self.frame_center() - self.coord_system.x_axis * (self.frame_width / 2.0)
            + self.coord_system.y_axis * (self.frame_height / 2.0)
    }

    fn render_to(canvas: &mut Image) {

    }
}
