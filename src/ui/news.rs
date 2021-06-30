use fltk::{
    enums::{Color, FrameType, Shortcut},
    frame::Frame,
    group::{Pack, PackType},
    menu::{MenuBar, MenuFlag},
    prelude::*,
    table::{TableRow, TableRowSelectMode},
    window::Window,
};

use super::app::Options;

/// Create a News Pane
pub fn new(window: &Window, options: &Options) -> Pack {
    let mut news = Pack::default()
        .with_pos(options.feeds_width + options.vertical_border_width, 0)
        .with_size(
            window.width() - options.feeds_width - options.vertical_border_width,
            window.height(),
        );
    news.set_type(PackType::Horizontal);
    news
}

/// Create a News Pack (supposed to be a child of the News Pane)
pub fn pack(window: &Window, options: &Options) -> Pack {
    Pack::default().with_size(
        window.width() - options.feeds_width - 2 * options.vertical_border_width,
        0,
    )
}

/// Create a News' Vertical Border (supposed to be a child of the News Pane)
pub fn vertical_border(options: &Options) -> Frame {
    let mut vertical_border = Frame::default().with_size(options.vertical_border_width, 0);
    vertical_border.set_frame(FrameType::FlatBox);
    vertical_border.set_color(Color::from_hex(0xF0_F0_F0));
    vertical_border
}

/// Create a Menu Bar (supposed to be a child of the News Pack)
pub fn menubar(options: &Options) -> MenuBar {
    let mut news_menubar = MenuBar::default().with_size(0, options.menubar_height);
    news_menubar.end();

    news_menubar.add(
        "@circle",
        Shortcut::from_char('r'),
        MenuFlag::Normal,
        |_| println!("Read pressed!"),
    );

    news_menubar
}

/// Create a News' Horizontal Border (supposed to be a child of the News Pack)
pub fn horizontal_border(options: &Options) -> Frame {
    let mut horizontal_border = Frame::default().with_size(0, options.horizontal_border_height);
    horizontal_border.set_frame(FrameType::FlatBox);
    horizontal_border.set_color(Color::from_hex(0xF0_F0_F0));
    horizontal_border
}

/// Create a News Feed (supposed to be a child of the News Pack)
pub fn feed(window: &Window, options: &Options) -> TableRow {
    let mut news_feed = TableRow::default().with_size(
        0,
        window.height() - options.menubar_height - options.horizontal_border_height,
    );
    news_feed.set_type(TableRowSelectMode::Multi);
    news_feed.end();
    news_feed
}
