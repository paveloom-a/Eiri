// Switch from the console subsystem to the windows subsystem in the release mode
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod ui;

use fltk::{
    app::{self, App},
    enums::{FrameType, Shortcut},
    image::PngImage,
    menu::MenuFlag,
    prelude::*,
};

fn main() {
    const FEEDS_WIDTH: i32 = 200;
    const MENUBAR_HEIGHT: i32 = 30;

    let window_icon: PngImage =
        PngImage::from_data(include_bytes!("../assets/eiri-32.png")).unwrap();

    let app = App::default();
    app::background(255, 255, 255);
    app::set_visible_focus(false);
    app::set_frame_type(FrameType::BorderBox);

    // 1. Window

    let mut window = ui::window::new(&window_icon);

    // 2. Window Tile

    let window_tile = ui::window::tile(&window);

    // 2.1 Feeds

    let feeds = ui::feeds::new(&window, FEEDS_WIDTH);

    // 2.1.1 Feeds MenuBar

    let mut feeds_menubar = ui::feeds::menubar(MENUBAR_HEIGHT);

    // 2.1.2 Feeds Tree

    let feeds_tree = ui::feeds::tree(&window, MENUBAR_HEIGHT);

    feeds.resizable(&feeds_menubar);
    feeds.resizable(&feeds_tree);
    feeds.end();

    // Logic

    ui::feeds::menubar_buttons(&mut feeds_menubar, feeds_tree, window_icon);

    // 2.2 News

    let news = ui::news::new(&window, FEEDS_WIDTH);

    // 2.2.1 News MenuBar

    let mut news_menubar = ui::news::menubar(MENUBAR_HEIGHT);
    news_menubar.add(
        "@circle",
        Shortcut::from_char('r'),
        MenuFlag::Normal,
        |_| println!("Read!"),
    );

    // 2.2.2 News Feed

    let news_feed = ui::news::feed(&window, MENUBAR_HEIGHT);

    news.resizable(&news_menubar);
    news.resizable(&news_feed);
    news.end();

    window_tile.end();
    window.end();
    window.show();

    app.run().unwrap();
}
