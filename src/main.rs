// Switch from the console subsystem to the windows subsystem in the release mode
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod events;
mod ui;

use fltk::{app, prelude::*};

fn main() {
    // Application channels
    let channels = ui::app::Channels::default();

    // Override the system's screen scaling
    for i in 0..app::screen_count() {
        app::set_screen_scale(i, 1.0);
    }

    // 0. App
    let app = ui::app::new();

    // 1. Window
    let window_icon = ui::windows::icon();
    let mut window = ui::windows::main(&window_icon);

    // 2. Window Tile
    let window_tile = ui::windows::tile(&window);

    // 2.1 Feeds Pane
    let feeds = ui::feeds::new(&window);

    // 2.1.1 Feeds Pack
    let feeds_pack = ui::feeds::pack();

    // 2.1.1.1 Feeds MenuBar
    let feeds_menubar = ui::feeds::menubar(&channels);

    // 2.1.1.2 Feeds Tree
    let feeds_tree = ui::feeds::tree(&channels);

    feeds_pack.resizable(&feeds_menubar);
    feeds_pack.resizable(&feeds_tree);
    feeds_pack.end();

    // 2.1.2 Feeds' Vertical Border
    let _feeds_vertical_border = ui::feeds::vertical_border();

    feeds.resizable(&feeds_pack);
    feeds.end();

    // 2.2 News Pane
    let news = ui::news::new(&window);

    // 2.2.1 News' Vertical Border
    let _news_vertical_border = ui::news::vertical_border();

    // 2.2.2 News Pack
    let news_pack = ui::news::pack(&window);

    // 2.2.2.1 News MenuBar
    let news_menubar = ui::news::menubar();

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
    let add_feed_window = ui::windows::add_feed(&window_icon, &channels);

    // 2. Add Folder Window
    let add_folder_window = ui::windows::add_folder(&window_icon, &channels);

    // Start the event loop
    while app.wait() {
        // Retranslation of signals between windows
        if let Ok(event) = channels.mw_signal_receiver.try_recv() {
            app::handle_main(event).ok();
        };
        if let Ok(event) = channels.mw_a_feed_w_translator.try_recv() {
            app::handle(event, &add_feed_window).ok();
        }
        if let Ok(event) = channels.mw_a_folder_w_translator.try_recv() {
            app::handle(event, &add_folder_window).ok();
        }
    }
}
