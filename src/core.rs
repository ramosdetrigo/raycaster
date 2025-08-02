use raylib_sys;

/// Trace log level
/// NOTE: Organized by priority level
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TraceLogLevel {
    /// Display all logs
    All = 0,
    /// Trace logging, intended for internal use only
    Trace = 1,
    /// Debug logging, used for internal debugging, it should be disabled on release builds
    Debug = 2,
    /// (DEFAULT) Info logging, used for program execution info
    Info = 3,
    /// Warning logging, used on recoverable failures
    Warning = 4,
    /// Error logging, used on unrecoverable failures
    Error = 5,
    /// Fatal logging, used to abort program: exit(EXIT_FAILURE)
    Fatal = 6,
    /// Disable logging
    None = 7,
}

/// Initializes a window and OpenGL context with specified size and title.
pub fn init_window(width: i32, height: i32, title: &str) {
    unsafe {
        let vec_u8_title = title.to_string().into_bytes();
        let c_title = vec_u8_title.as_ptr().cast();
        raylib_sys::InitWindow(width, height, c_title);
    }
}

/// Closes window and unloads OpenGL context.
pub fn close_window() {
    unsafe { raylib_sys::CloseWindow() }
}

/// True if window should close - ExitKey pressed (default is ESC),
/// Window exit button clicked/Alt+F4 pressed, etc.
pub fn window_should_close() -> bool {
    unsafe { raylib_sys::WindowShouldClose() }
}

/// Sets target max FPS
pub fn set_target_fps(fps: i32) {
    unsafe { raylib_sys::SetTargetFPS(fps) }
}

/// Set the current threshold (minimum) log level (Default: LogInfo)
pub fn set_tracelog_level(log_level: TraceLogLevel) {
    unsafe { raylib_sys::SetTraceLogLevel(log_level as i32) }
}
