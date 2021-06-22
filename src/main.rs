// Switch from the console subsystem to the windows subsystem in the release mode
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use fltk::{app, image::PngImage, prelude::*, window::Window};

fn main() {
    let icon: PngImage = PngImage::from_data(include_bytes!("../assets/eiri-32.png")).unwrap();
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Eiri");
    wind.set_icon(Some(icon));
    wind.end();
    wind.show();
    app.run().unwrap();
}
