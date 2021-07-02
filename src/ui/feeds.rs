use fltk::{
    app::{self, Receiver},
    button::Button,
    enums::{Align, CallbackTrigger, Color, FrameType, Shortcut},
    frame::Frame,
    group::{Pack, PackType},
    image::PngImage,
    input::Input,
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
    let mut feeds_menubar = MenuBar::default().with_size(0, options.menubar_height);
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

    feeds_menubar
}

/// Create the Add Feed Window (created by the Add Feed Button in the Menu Bar)
pub fn add_feed_window(window_icon: &PngImage) -> (Window, Receiver<String>) {
    // Channel 1: Feeds Tree and the Add Feed Window's Input Widget / OK Button
    let (s_1, r) = app::channel::<String>();
    let s_2 = s_1.clone();

    // 1. Window
    let mut window = Window::default()
        .with_size(300, 200)
        .center_screen()
        .with_label("Add feed");
    window.size_range(300, 200, 0, 200);
    window.set_icon(Some(window_icon.clone()));
    window.make_resizable(true);
    window.make_modal(true);

    window.handle(move |w, ev| {
        let mut rv: bool = false;
        match ev.bits() {
            events::SHOW_ADD_FEED_WINDOW => {
                w.show();
                rv = true;
            }
            events::HIDE_ADD_FEED_WINDOW => {
                w.hide();
                rv = true;
            }
            _ => (),
        }
        rv
    });

    // 1.1 Input
    let mut input = Input::new(20, 50, window.width() - 40, 25, "Feed URL:");
    input.set_align(Align::TopLeft);
    input.set_frame(FrameType::BorderBox);
    input.set_trigger(CallbackTrigger::EnterKeyAlways);
    window.resizable(&input);

    input.set_callback(move |i| {
        app::handle_main(events::HIDE_ADD_FEED_WINDOW).unwrap();
        s_1.send(i.value());
        println!("Sent the path.");
        i.set_value("");
    });

    // 1.2 Buttons' Pack
    let mut buttons_pack = Pack::default()
        .with_pos(195, window.height() - 40)
        .with_size(85, 25);
    buttons_pack.set_type(PackType::Horizontal);

    // 1.2.1 Resizable Box
    let resizable_box = Frame::default().with_size(5, 0);

    // 1.2.2 OK Button
    let mut ok_button = Button::default().with_size(80, 0).with_label("OK");

    ok_button.set_callback(move |_| {
        app::handle_main(events::HIDE_ADD_FEED_WINDOW).unwrap();
        s_2.send(input.value());
        input.set_value("");
    });

    buttons_pack.end();
    buttons_pack.resizable(&resizable_box);

    window.end();
    (window, r)
}

/// Create a Feeds' Horizontal Border (supposed to be a child of the Feeds Pack)
pub fn horizontal_border(options: &Options) -> Frame {
    let mut horizontal_border = Frame::default().with_size(0, options.horizontal_border_height);
    horizontal_border.set_frame(FrameType::FlatBox);
    horizontal_border.set_color(Color::from_hex(0xF0_F0_F0));
    horizontal_border
}

/// Create a Feeds Tree (supposed to be a child of the Feeds Pack)
pub fn tree(window: &Window, options: &Options) -> Tree {
    let mut feeds_tree = Tree::default().with_size(
        0,
        window.height() - options.menubar_height - options.horizontal_border_height,
    );
    feeds_tree.set_frame(FrameType::FlatBox);
    feeds_tree.set_show_root(false);
    feeds_tree.end();
    feeds_tree
}
