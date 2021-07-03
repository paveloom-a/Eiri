// Switch from the console subsystem to the windows subsystem in the release mode
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod events;
mod ui;

use fltk::{app, prelude::*};

fn main() {
    // The app's Options
    const OPTIONS: ui::app::Options = ui::app::Options {
        window_min_width: 1000,
        window_min_height: 600,
        feeds_width: 200,
        menubar_height: 30,
        vertical_border_width: 2,
        horizontal_border_height: 4,
    };

    // Override the system's screen scaling
    for i in 0..app::screen_count() {
        app::set_screen_scale(i, 1.0);
    }

    // 0. App
    let app = ui::app::new();

    // 1. Window
    let window_icon = ui::windows::icon();
    let mut window = ui::windows::main(&window_icon, &OPTIONS);

    // 2. Window Tile
    let window_tile = ui::windows::tile(&window, &OPTIONS);

    // 2.1 Feeds Pane
    let feeds = ui::feeds::new(&window, &OPTIONS);

    // 2.1.1 Feeds Pack
    let feeds_pack = ui::feeds::pack(&OPTIONS);

    // 2.1.1.1 Feeds MenuBar
    let feeds_menubar = ui::feeds::menubar(&OPTIONS);

    // 2.1.1.2 Feeds Tree
    let mut feeds_tree = ui::feeds::tree();

    feeds_pack.resizable(&feeds_menubar);
    feeds_pack.resizable(&feeds_tree);
    feeds_pack.end();

    // 2.1.2 Feeds' Vertical Border
    let _feeds_vertical_border = ui::feeds::vertical_border(&OPTIONS);

    feeds.resizable(&feeds_pack);
    feeds.end();

    // 2.2 News Pane
    let news = ui::news::new(&window, &OPTIONS);

    // 2.2.1 News' Vertical Border
    let _news_vertical_border = ui::news::vertical_border(&OPTIONS);

    // 2.2.2 News Pack
    let news_pack = ui::news::pack(&window, &OPTIONS);

    // 2.2.2.1 News MenuBar
    let news_menubar = ui::news::menubar(&OPTIONS);

    // 2.2.2.2 News Feed
    let news_feed = ui::news::feed();

    news_pack.resizable(&news_menubar);
    news_pack.resizable(&news_feed);
    news_pack.end();

    news.resizable(&news_pack);
    news.end();

    window_tile.end();

    window.end();
    window.show();

    // Hidden windows

    // 1. Add Feed Window
    let (add_feed_window, r_ch1) = ui::windows::add_feed(&window_icon);

    // Redirect the signals to other windows
    window.handle(move |_, ev| {
        let ev = ev.bits();
        if ev == events::SHOW_ADD_FEED_WINDOW {
            app::handle(ev, &add_feed_window).ok();
            true
        } else {
            false
        }
    });

    // Start the event loop
    while app.wait() {
        if let Ok(path) = r_ch1.try_recv() {
            feeds_tree.add(path.as_str());
            feeds_tree.redraw();
        }
    }
}
