// Switch from the console subsystem to the windows subsystem in the release mode
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use fltk::{
    app::{self, App},
    enums::{FrameType, Shortcut},
    frame::Frame,
    group::{Pack, Tile},
    image::PngImage,
    menu::{MenuBar, MenuFlag},
    prelude::*,
    tree::Tree,
    window::Window,
};

fn main() {
    let app = App::default();
    app::background(255, 255, 255);
    app::set_visible_focus(false);
    app::set_frame_type(FrameType::NoBox);

    // 1. Window

    let mut window = Window::new(100, 100, 1000, 600, "Eiri");
    let window_icon = PngImage::from_data(include_bytes!("../assets/eiri-32.png")).unwrap();
    window.set_icon(Some(window_icon));
    window.size_range(1000, 600, 0, 0);
    window.make_resizable(true);

    // 2. Window Tile

    let window_tile = Tile::default().with_size(window.width(), window.height());
    let mut window_tile_resize_box = Frame::default()
        .with_pos(window_tile.x() + 200, window_tile.y())
        .with_size(window_tile.w() - 800, window_tile.h());
    window_tile_resize_box.hide();
    window_tile.resizable(&window_tile_resize_box);

    // 2.1 Feeds

    let feeds_width = 200;
    let feeds_toolbar_height = 30;

    let mut feeds = Pack::default().with_size(feeds_width, window.height());
    feeds.set_frame(FrameType::UpFrame);

    // 2.1.1 Feeds MenuBar

    let mut feeds_menubar = MenuBar::default().with_size(0, feeds_toolbar_height);
    feeds_menubar.add("@+", Shortcut::from_char('a'), MenuFlag::Normal, |_| {
        println!("Pressed!")
    });
    feeds_menubar.end();

    // 2.1.2 Feeds Tree

    let feeds_tree = Tree::default().with_size(0, window.height() - feeds_toolbar_height);
    feeds_tree.end();

    feeds.resizable(&feeds_tree);
    feeds.end();

    // 2.2 News

    let news = Pack::default()
        .with_pos(feeds_width, 0)
        .with_size(window.height() - feeds_width, window.height());

    news.end();

    window_tile.end();
    window.end();
    window.show();

    app.run().unwrap();
}
