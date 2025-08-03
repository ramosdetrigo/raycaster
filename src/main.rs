use std::slice;

use glam::DVec3;
use raycaster::{
    lights::Point,
    objects::{Cilinder, Cone, Plane, Sphere},
    *,
};
// use raycaster;
use raylib::prelude::*;

const DIRECTIONS: [(KeyboardKey, DVec3); 6] = [
    (KeyboardKey::KEY_W, DVec3::NEG_Z),
    (KeyboardKey::KEY_S, DVec3::Z),
    (KeyboardKey::KEY_A, DVec3::NEG_X),
    (KeyboardKey::KEY_D, DVec3::X),
    (KeyboardKey::KEY_SPACE, DVec3::Y),
    (KeyboardKey::KEY_LEFT_SHIFT, DVec3::NEG_Y),
];

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("Hello, World")
        .log_level(TraceLogLevel::LOG_NONE)
        .build();
    // rl.set_target_fps(60);

    // Criando os objetos da cena
    let mut camera = raycaster::Camera::new(DVec3::new(0.0, 0.0, 0.0), 1.6, 0.9, 0.8);
    let ball = Sphere::new(DVec3::new(-2.0, 2.0, -16.0), 4.0, raycaster::Material::WHITE);

    let cilinder = Cilinder::new(
        DVec3::new(4.0, 4.0, -16.0),
        (DVec3::X - DVec3::Z - DVec3::Y).normalize(),
        8.0,
        2.0,
        true,
        true,
        raycaster::Material::WHITE,
    );

    let cone = Cone::new(
        DVec3::new(-8.0, 4.0, -16.0),
        -(DVec3::X - DVec3::Z - DVec3::Y).normalize(),
        4.0,
        2.0,
        true,
        raycaster::Material::WHITE,
    );

    let plane = Plane::new(DVec3::new(0.0, -2.0, 0.0), DVec3::Y, raycaster::Material::WHITE);

    let light = Point::new(DVec3::new(0.0, 6.0, -10.0), DVec3::new(1.0, 0.65, 0.7), 0.5);
    let scene = Scene {
        objects: vec![Box::new(ball), Box::new(cilinder), Box::new(plane), Box::new(cone)],
        lights: vec![Box::new(light)],
        ambient_light: DVec3::splat(0.2),
    };

    // Cria um novo canvas para desenhar a cena
    let mut canvas = Image::gen_image_color(800, 450, Color::BLACK);
    canvas.set_format(PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8);
    // Converte numa textura da GPU
    let mut texture = rl.load_texture_from_image(&thread, &canvas).unwrap();
    let pixel_data = unsafe {
        slice::from_raw_parts_mut(
            canvas.data() as *mut u8,
            (canvas.width * canvas.height * 3) as usize,
        )
    };

    while !rl.window_should_close() {
        // Calcula o movimento usando WASD + Shift/Espaço
        let movement: DVec3 = DIRECTIONS
            .iter()
            .filter(|(key, _)| rl.is_key_down(*key))
            .map(|(_, dir)| *dir)
            .sum();
        if movement != DVec3::ZERO {
            camera.p0 += movement.normalize() * 0.1;
        }

        // Renderiza o frame atual no canvas na CPU
        camera.render_scene_to(&scene, &mut canvas);
        // Atualiza a textura da GPU com os dados da imagem
        texture.update_texture(pixel_data).unwrap();

        // Desenha a textura na tela
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
        // Contador de FPS
        d.draw_text(
            format!("{}", d.get_fps()).as_str(),
            10,
            10,
            20,
            Color::WHITE,
        );
    }
}
