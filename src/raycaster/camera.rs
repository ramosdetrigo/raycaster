use std::slice;

use glam::{DMat3, DVec3, U8Vec3};
use raylib::{color::Color, ffi::PixelFormat, texture::Image};
use rayon::prelude::*;

use crate::{Ray, Scene};

/// Camera com observador centrado em p0, sistema de coordenadas definido,
/// e frame de rendering
pub struct Camera {
    pub p0: DVec3,
    pub frame_width: f64,
    pub frame_height: f64,
    pub frame_distance: f64,
    pub coord_system: DMat3,
}

impl Camera {
    /// Construtor da câmera
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
        self.p0 - self.frame_distance * self.coord_system.z_axis
    }

    /// Ponto correspondente à quina superior-esquerda do frame
    fn frame_00(&self) -> DVec3 {
        self.frame_center()
            - self.coord_system.x_axis * (self.frame_width / 2.0)
            + self.coord_system.y_axis * (self.frame_height / 2.0)
    }

    /// Cria um novo canvas (struct imagem do raylib), renderiza a cena nele e o retorna.
    #[must_use]
    pub fn render_scene(&self, scene: &Scene, x_res: i32, y_res: i32) -> Image {
        let mut image = Image::gen_image_color(x_res, y_res, Color::BLACK);
        image.set_format(PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8);
        self.render_scene_to(scene, &mut image);
        image
    }

    /// Desenha a cena para um canvas (struct imagem do raylib)
    pub fn render_scene_to(&self, scene: &Scene, canvas: &mut Image) {
        if canvas.format != PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8 as i32 {
            panic!("Camera can only render to pixel format R8G8B8.");
        }

        // Calcula o dx e o dy com base na resolução do Canvas
        let res_x = canvas.width;
        let res_y = canvas.height;
        let dx = (self.frame_width / res_x as f64) * self.coord_system.x_axis;
        let dy = -(self.frame_height / res_y as f64) * self.coord_system.y_axis;

        // Centro do primeiro quadrado da grade de rendering
        let p00 = self.frame_00() + dx / 2.0 + dy / 2.0;

        // Array de pixels bruto do canvas
        let pixel_data = unsafe {
            slice::from_raw_parts_mut(canvas.data() as *mut u8, (res_x * res_y * 3) as usize)
        };

        // Renderiza cada pixel (cada trio de u8 em pixel_data)
        pixel_data
            .par_chunks_mut(3) // Renderiza em paralelo usando a biblioteca Rayon
            .enumerate()
            .for_each(|(i, pixel)| {
                // Pixel (px,py) atual
                let px = (i % res_x as usize) as f64;
                let py = (i / res_x as usize) as f64;

                // Calcula a direção do próximo raio com base no pixel atual e p00
                let p_target = p00 + px * dx + py * dy;
                let ray_dr = (p_target - self.p0).normalize();
                let ray = Ray::new(self.p0, ray_dr);

                // Pega a interseção mais próxima
                let closest_intersection = &scene
                    .objects
                    .iter()
                    .filter_map(|object| object.intersects(&ray)) // Só as interseções que não são "None"
                    .min_by(|intersection1, intersection2| {
                        intersection1.t.total_cmp(&intersection2.t)
                    });

                // Se houve interseção, pinta a cor da iluminação nela
                // Se não, pinta o pixel de preto.
                let mut total_light = U8Vec3::ZERO;
                if let Some(intersection) = closest_intersection {
                    // Iluminação "passiva" - luz ambiente
                    let passive = scene.ambient_light * intersection.material.k_amb;
                    // Iluminação ativa - as luzes definidas do cenário
                    let active: DVec3 = scene
                        .lights
                        .iter()
                        .map(|light| light.color_at(&intersection, ray_dr, &scene))
                        .sum();
                    // Converte pra um número entre 0 e 255 pro valor de cor
                    total_light = ((passive + active) * 255.0)
                        .min(DVec3::splat(255.0)) // A cor deve ser no máximo 255
                        .as_u8vec3();
                }

                pixel[0] = total_light.x;
                pixel[1] = total_light.y;
                pixel[2] = total_light.z;
            });
    }
}
