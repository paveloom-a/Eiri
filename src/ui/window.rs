use fltk::{
    frame::Frame,
    group::Tile,
    image::PngImage,
    prelude::*,
    window::{DoubleWindow, Window},
};

/// Create a Window
pub fn new(window_icon: &PngImage) -> DoubleWindow {
    let mut window = Window::new(100, 100, 1000, 600, "Eiri");
    window.set_icon(Some(window_icon.clone()));
    window.size_range(1000, 600, 0, 0);
    window.make_resizable(true);
    window
}

/// Create a Window Tile (supposed to be a child of the Window)
pub fn tile(window: &Window) -> Tile {
    let window_tile = Tile::default().with_size(window.width(), window.height());
    let mut window_tile_resize_box = Frame::default()
        .with_pos(window_tile.x() + 200, window_tile.y())
        .with_size(window_tile.w() - 800, window_tile.h());
    window_tile_resize_box.hide();
    window_tile.resizable(&window_tile_resize_box);
    window_tile
}
