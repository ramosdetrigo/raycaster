use raylib_sys;

/// Setup canvas (framebuffer) to start drawing
pub fn begin_drawing() {
    unsafe { raylib_sys::BeginDrawing() }
}

/// End canvas drawing and swap buffers (double buffering)
pub fn end_drawing() {
    unsafe { raylib_sys::EndDrawing() }
}
