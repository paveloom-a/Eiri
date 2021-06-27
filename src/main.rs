// Switch from the console subsystem to the windows subsystem in the release mode
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod events;
mod ui;

use fltk::{
    app::{self, App},
    enums::FrameType,
    image::PngImage,
    prelude::*,
};

fn main() {
    const FEEDS_WIDTH: i32 = 200;
    const MENUBAR_HEIGHT: i32 = 30;

    // Channel 1: Feeds Tree and the Add Feed Window's Input Widget / OK Button
    let (s_ch1, r_ch1) = app::channel::<String>();

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
    let feeds_menubar = ui::feeds::menubar(MENUBAR_HEIGHT);

    // 2.1.2 Feeds Tree
    let mut feeds_tree = ui::feeds::tree(&window, MENUBAR_HEIGHT);

    feeds.resizable(&feeds_menubar);
    feeds.resizable(&feeds_tree);
    feeds.end();

    // 2.2 News
    let news = ui::news::new(&window, FEEDS_WIDTH);

    // 2.2.1 News MenuBar
    let news_menubar = ui::news::menubar(MENUBAR_HEIGHT);

    // 2.2.2 News Feed
    let news_feed = ui::news::feed(&window, MENUBAR_HEIGHT);

    news.resizable(&news_menubar);
    news.resizable(&news_feed);
    news.end();

    window_tile.end();
    window.end();
    window.show();

    // Hidden windows

    // 1. Add Feed Window
    let add_feed_window = ui::feeds::add_feed_window(&s_ch1, &window_icon);

    window.handle(move |_, ev| {
        let mut rv: bool = false;
        let ev = ev.bits();
        if ev == events::SHOW_ADD_FEED_WINDOW {
            app::handle(ev, &add_feed_window).unwrap();
            rv = true;
        }
        rv
    });

    while app.wait() {
        if let Some(path) = r_ch1.recv() {
            feeds_tree.add(path.as_str());
            feeds_tree.redraw();
        }
    }
}
