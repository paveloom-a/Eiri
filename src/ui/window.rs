use fltk::{
    frame::Frame,
    group::Tile,
    image::PngImage,
    prelude::*,
    window::{DoubleWindow, Window},
};

use super::app::Options;

/// Load a Window Icon
pub fn icon() -> PngImage {
    PngImage::from_data(include_bytes!("../../assets/eiri-32.png")).unwrap()
}

/// Create a Window
pub fn new(window_icon: &PngImage, options: &Options) -> DoubleWindow {
    let mut window = Window::new(
        100,
        100,
        options.window_min_width,
        options.window_min_height,
        "Eiri",
    );
    window.set_icon(Some(window_icon.clone()));
    window.size_range(options.window_min_width, options.window_min_height, 0, 0);
    window.make_resizable(true);
    window
}

/// Create a Window Tile (supposed to be a child of the Window)
pub fn tile(window: &Window, options: &Options) -> Tile {
    let window_tile = Tile::default().with_size(window.width(), window.height());
    let mut window_tile_resize_box = Frame::default()
        .with_pos(
            window_tile.x() + options.feeds_width + options.vertical_border_width,
            window_tile.y(),
        )
        .with_size(window_tile.w() - 800, window_tile.h());
    window_tile_resize_box.hide();
    window_tile.resizable(&window_tile_resize_box);
    window_tile
}
