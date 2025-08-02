mod vec3;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(1920, 1080).title("Hello, World").build();
    rl.set_trace_log(TraceLogLevel::LOG_NONE);
    // rl.set_target_fps(60);

    let image = Image::gen_image_color(1920, 1080, Color::RED);
    
    while !rl.window_should_close() {
        let texture = rl.load_texture_from_image(&thread, &image).unwrap();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_texture(&texture, 256, 0, Color::WHITE);
        d.draw_text(format!("{}", d.get_fps()).as_str(), 12, 12, 20, Color::BLACK);
    }
}
