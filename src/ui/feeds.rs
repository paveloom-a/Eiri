use fltk::{
    app,
    enums::{Color, FrameType, Shortcut},
    frame::Frame,
    group::{Pack, PackType},
    menu::{MenuBar, MenuFlag},
    prelude::*,
    tree::Tree,
    window::Window,
};

use super::app::Options;
use crate::events;

/// Create a Feeds Pane
pub fn new(window: &Window, options: &Options) -> Pack {
    let mut feeds = Pack::default().with_size(
        options.feeds_width + options.vertical_border_width,
        window.height(),
    );
    feeds.set_type(PackType::Horizontal);
    feeds
}

/// Create a Feeds Pack (supposed to be a child of the Feeds Pane)
pub fn pack(options: &Options) -> Pack {
    Pack::default().with_size(options.feeds_width, 0)
}

/// Create a Feeds' Vertical Border (supposed to be a child of the Feeds Pane)
pub fn vertical_border(options: &Options) -> Frame {
    let mut vertical_border = Frame::default().with_size(options.vertical_border_width, 0);
    vertical_border.set_frame(FrameType::FlatBox);
    vertical_border.set_color(Color::from_hex(0xF0_F0_F0));
    vertical_border
}

/// Create a Menu Bar (supposed to be a child of the Feeds Pack)
pub fn menubar(options: &Options) -> MenuBar {
    let _top_border = horizontal_border(options);

    let mut feeds_menubar = MenuBar::default().with_size(0, options.menubar_height);
    feeds_menubar.set_frame(FrameType::FlatBox);
    feeds_menubar.end();

    feeds_menubar.add(
        "@#+/Add Feed...",
        Shortcut::from_char('a'),
        MenuFlag::Normal,
        |_| {
            app::handle_main(events::SHOW_ADD_FEED_WINDOW).unwrap();
        },
    );

    feeds_menubar.add(
        "@#+/Add Folder...",
        Shortcut::from_char('f'),
        MenuFlag::Normal,
        |_| println!("Add Folder pressed!"),
    );

    let _bottom_border = horizontal_border(options);

    feeds_menubar
}

/// Create a Feeds' Horizontal Border (supposed to be a child of the Feeds Pack)
pub fn horizontal_border(options: &Options) -> Frame {
    let mut horizontal_border = Frame::default().with_size(0, options.horizontal_border_height);
    horizontal_border.set_frame(FrameType::FlatBox);
    horizontal_border.set_color(Color::from_hex(0xF0_F0_F0));
    horizontal_border
}

/// Create a Feeds Tree (supposed to be a child of the Feeds Pack)
pub fn tree() -> Tree {
    let mut feeds_tree = Tree::default();
    feeds_tree.set_frame(FrameType::FlatBox);
    feeds_tree.set_show_root(false);
    feeds_tree.end();
    feeds_tree
}
