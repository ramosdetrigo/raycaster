use glam::DVec3;
use raycaster::*;
// use raycaster;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Hello, World")
        .log_level(TraceLogLevel::LOG_NONE)
        .build();
    // rl.set_target_fps(60);

    let camera = raycaster::Camera::new(DVec3::ZERO, 1.6, 0.9, 0.8);
    let ball = raycaster::objects::Sphere::new(
        DVec3::new(0.0, 0.0, -16.0),
        4.0,
        raycaster::Material::WHITE,
    );
    let scene = Scene {
        objects: vec![ball],
    };

    let img = camera.render_scene(&scene, 800, 600);
    let texture = rl.load_texture_from_image(&thread, &img).unwrap();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
        d.draw_text(
            format!("{}", d.get_fps()).as_str(),
            12,
            12,
            20,
            Color::BLACK,
        );
    }
}
