// Switch from the console subsystem to the windows subsystem in the release mode
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use fltk::{app, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from Rust");
    wind.end();
    wind.show();
    app.run().unwrap();
}
