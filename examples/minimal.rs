use raylib::*;

fn main() {
    core::set_tracelog_level(core::TraceLogLevel::None);
    core::init_window(600, 400, "Hello, World!");
    dbg!("Window initialized.");
    
    core::set_target_fps(60);
    dbg!("Set target fps.");
    while !raylib::core::window_should_close() {
        drawing::begin_drawing();
        drawing::end_drawing();
    }
    
    core::close_window();
}
