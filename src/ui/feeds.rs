use fltk::{
    app::event_key,
    enums::{Color, Event, FrameType, Key, Shortcut},
    frame::Frame,
    group::{Pack, PackType},
    menu::{MenuBar, MenuFlag},
    prelude::*,
    tree::{Tree, TreeItemReselectMode, TreeReason, TreeSelect},
    window::Window,
};

use super::app::{Channels, OPTIONS};
use crate::events;

/// Create a Feeds Pane
pub fn new(window: &Window) -> Pack {
    let mut feeds = Pack::default().with_size(
        OPTIONS.feeds_width + OPTIONS.vertical_border_width,
        window.height(),
    );
    feeds.set_type(PackType::Horizontal);
    feeds
}

/// Create a Feeds Pack (supposed to be a child of the Feeds Pane)
pub fn pack() -> Pack {
    Pack::default().with_size(OPTIONS.feeds_width, 0)
}

/// Create a Feeds' Vertical Border (supposed to be a child of the Feeds Pane)
pub fn vertical_border() -> Frame {
    let mut vertical_border = Frame::default().with_size(OPTIONS.vertical_border_width, 0);
    vertical_border.set_frame(FrameType::FlatBox);
    vertical_border.set_color(Color::from_hex(0xF0_F0_F0));
    vertical_border
}

/// Create a Menu Bar (supposed to be a child of the Feeds Pack)
pub fn menubar(channels: &Channels) -> MenuBar {
    let _top_border = horizontal_border();

    let mut feeds_menubar = MenuBar::default().with_size(0, OPTIONS.menubar_height);
    feeds_menubar.set_frame(FrameType::FlatBox);
    feeds_menubar.end();

    feeds_menubar.add(
        "@#+/Add Feed\t",
        Shortcut::from_char('a'),
        MenuFlag::Normal,
        {
            let s = channels.add_feed_window.s.clone();
            move |_| {
                s.try_send(events::SHOW_ADD_FEED_WINDOW).ok();
            }
        },
    );

    feeds_menubar.add(
        "@#+/Add Folder\t",
        Shortcut::from_char('f'),
        MenuFlag::Normal,
        {
            let s = channels.add_folder_window.s.clone();
            move |_| {
                s.try_send(events::SHOW_ADD_FOLDER_WINDOW).ok();
            }
        },
    );

    let _bottom_border = horizontal_border();

    feeds_menubar
}

/// Create a Feeds' Horizontal Border (supposed to be a child of the Feeds Pack)
pub fn horizontal_border() -> Frame {
    let mut horizontal_border = Frame::default().with_size(0, OPTIONS.horizontal_border_height);
    horizontal_border.set_frame(FrameType::FlatBox);
    horizontal_border.set_color(Color::from_hex(0xF0_F0_F0));
    horizontal_border
}

/// Create a Feeds Tree (supposed to be a child of the Feeds Pack)
pub fn tree(channels: &Channels) -> Tree {
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

    feeds_tree.handle({
        let add_feed_receiver = channels.add_feed.r.clone();
        let add_folder_receiver = channels.add_folder.r.clone();
        move |t, ev| match ev {
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
            _ => match ev.bits() {
                events::ADD_FEED_EVENT => {
                    if let Ok(path) = add_feed_receiver.try_recv() {
                        t.add(path.as_str());
                        t.redraw();
                        true
                    } else {
                        false
                    }
                }
                events::ADD_FOLDER_EVENT => {
                    if let Ok(path) = add_folder_receiver.try_recv() {
                        t.add(path.as_str());
                        t.redraw();
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
        }
    });

    feeds_tree
}
