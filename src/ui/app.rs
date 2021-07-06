use fltk::{
    app::{self, App},
    enums::FrameType,
};

/// A struct providing access to the application's constants
pub struct Constants {
    pub window_min_width: i32,
    pub window_min_height: i32,
    pub feeds_width: i32,
    pub menubar_height: i32,
    pub vertical_border_width: i32,
    pub horizontal_border_height: i32,
}

impl Constants {
    /// Get the default set of the application's constants
    const fn default() -> Constants {
        Constants {
            window_min_width: 1000,
            window_min_height: 600,
            feeds_width: 200,
            menubar_height: 30,
            vertical_border_width: 2,
            horizontal_border_height: 4,
        }
    }
}

/// Default set of the application's contstants
pub const CONSTANTS: Constants = Constants::default();

/// Create a new App
pub fn new() -> App {
    let app = App::default();
    app::background(255, 255, 255);
    app::set_frame_type(FrameType::BorderBox);
    app
}
