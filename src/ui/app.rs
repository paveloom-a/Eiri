use fltk::{
    app::{self, App},
    enums::FrameType,
};

/// The app's Options
pub struct Options {
    pub window_min_width: i32,
    pub window_min_height: i32,
    pub feeds_width: i32,
    pub menubar_height: i32,
    pub vertical_border_width: i32,
    pub horizontal_border_height: i32,
}

/// Create a new App
pub fn new() -> App {
    let app = App::default();
    app::background(255, 255, 255);
    app::set_visible_focus(false);
    app::set_frame_type(FrameType::BorderBox);
    app
}
