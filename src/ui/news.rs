use fltk::{
    enums::{Color, FrameType, Shortcut},
    group::Pack,
    menu::{MenuBar, MenuFlag},
    prelude::*,
    table::{TableRow, TableRowSelectMode},
    window::Window,
};

/// Create a News pane
pub fn new(window: &Window, feeds_width: i32) -> Pack {
    let mut news = Pack::default()
        .with_pos(feeds_width, 0)
        .with_size(window.width() - feeds_width, window.height());
    news.set_frame(FrameType::BorderFrame);
    news.set_color(Color::Black);
    news
}

/// Create a Menu Bar (supposed to be a child of the News pane)
pub fn menubar(menubar_height: i32) -> MenuBar {
    let mut news_menubar = MenuBar::default().with_size(0, menubar_height);
    news_menubar.end();

    news_menubar.add(
        "@circle",
        Shortcut::from_char('r'),
        MenuFlag::Normal,
        |_| println!("Read pressed!"),
    );

    news_menubar
}

/// Create a News Feed (supposed to be a child of the News pane)
pub fn feed(window: &Window, menubar_height: i32) -> TableRow {
    let mut news_feed = TableRow::default().with_size(0, window.height() - menubar_height);
    news_feed.set_type(TableRowSelectMode::Multi);
    news_feed.set_frame(FrameType::BorderBox);
    news_feed.end();
    news_feed
}
