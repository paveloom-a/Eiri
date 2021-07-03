use fltk::{
    app::{self, event_key},
    enums::{Color, Event, FrameType, Key, Shortcut},
    frame::Frame,
    group::{Pack, PackType},
    menu::{MenuBar, MenuFlag},
    prelude::*,
    tree::{Tree, TreeItemReselectMode, TreeReason, TreeSelect},
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
        "@#+/Add Feed\t",
        Shortcut::from_char('a'),
        MenuFlag::Normal,
        |_| {
            app::handle_main(events::SHOW_ADD_FEED_WINDOW).ok();
        },
    );

    feeds_menubar.add(
        "@#+/Add Folder\t",
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
    feeds_tree.set_select_mode(TreeSelect::SingleDraggable);
    feeds_tree.set_item_reselect_mode(TreeItemReselectMode::Always);
    feeds_tree.end();

    feeds_tree.set_callback(|t| {
        if t.select_mode() == TreeSelect::SingleDraggable
            && (t.callback_reason() == TreeReason::Selected
                || t.callback_reason() == TreeReason::Reselected)
        {
            if let Some(item) = &t.get_item_focus() {
                if let Some(label) = item.label() {
                    println!("Selected an item with label \"{}\".", label);
                }
            }
        }
    });

    feeds_tree.handle(|t, ev| match ev {
        Event::KeyDown => match event_key() {
            Key::ControlL | Key::ShiftL => {
                t.set_select_mode(TreeSelect::Multi);
                true
            }
            Key::Enter => {
                if let Some(item) = &t.get_item_focus() {
                    t.select_only(item, false).ok();
                    if let Some(label) = item.label() {
                        println!("Selected an item with label \"{}\".", label);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        },
        Event::KeyUp => {
            if event_key() == Key::ControlL | Key::ShiftL {
                t.set_select_mode(TreeSelect::SingleDraggable);
                true
            } else {
                false
            }
        }
        _ => false,
    });

    feeds_tree
}
