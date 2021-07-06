use fltk::{
    enums::{Color, FrameType, Shortcut},
    frame::Frame,
    group::{Pack, PackType},
    menu::{MenuBar, MenuFlag},
    prelude::*,
    table::{TableRow, TableRowSelectMode},
    window::Window,
};

use super::app::CONSTANTS;

/// Create a News Pane
pub fn new(window: &Window) -> Pack {
    let mut news = Pack::default()
        .with_pos(CONSTANTS.feeds_width + CONSTANTS.vertical_border_width, 0)
        .with_size(
            window.width() - CONSTANTS.feeds_width - CONSTANTS.vertical_border_width,
            window.height(),
        );
    news.set_type(PackType::Horizontal);
    news
}

/// Create a News Pack (supposed to be a child of the News Pane)
pub fn pack(window: &Window) -> Pack {
    Pack::default().with_size(
        window.width() - CONSTANTS.feeds_width - 2 * CONSTANTS.vertical_border_width,
        0,
    )
}

/// Create a News' Vertical Border (supposed to be a child of the News Pane)
pub fn vertical_border() -> Frame {
    let mut vertical_border = Frame::default().with_size(CONSTANTS.vertical_border_width, 0);
    vertical_border.set_frame(FrameType::FlatBox);
    vertical_border.set_color(Color::from_hex(0xF0_F0_F0));
    vertical_border
}

/// Create a Menu Bar (supposed to be a child of the News Pack)
pub fn menubar() -> MenuBar {
    let _top_border = horizontal_border();

    let mut news_menubar = MenuBar::default().with_size(0, CONSTANTS.menubar_height);
    news_menubar.set_frame(FrameType::FlatBox);
    news_menubar.end();

    news_menubar.add(
        "@circle",
        Shortcut::from_char('r'),
        MenuFlag::Normal,
        |_| println!("Read pressed!"),
    );

    let _bottom_border = horizontal_border();

    news_menubar
}

/// Create a News' Horizontal Border (supposed to be a child of the News Pack)
pub fn horizontal_border() -> Frame {
    let mut horizontal_border = Frame::default().with_size(0, CONSTANTS.horizontal_border_height);
    horizontal_border.set_frame(FrameType::FlatBox);
    horizontal_border.set_color(Color::from_hex(0xF0_F0_F0));
    horizontal_border
}

/// Create a News Feed (supposed to be a child of the News Pack)
pub fn feed() -> TableRow {
    let mut news_feed = TableRow::default();
    news_feed.set_type(TableRowSelectMode::Multi);
    news_feed.end();
    news_feed
}
