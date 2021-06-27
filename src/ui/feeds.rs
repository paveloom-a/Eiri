use crate::events;

use fltk::{
    app::{self, Sender},
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

/// Create a Feeds pane
pub fn new(window: &Window, feeds_width: i32) -> Pack {
    let mut feeds = Pack::default().with_size(feeds_width, window.height());
    feeds.set_frame(FrameType::BorderFrame);
    feeds.set_color(Color::Black);
    feeds
}

/// Create the Add Feed Window (created by the Add Feed Button in the Menu Bar)
pub fn add_feed_window(s: &Sender<String>, window_icon: &PngImage) -> Window {
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

    let s_1 = s.clone();
    input.set_callback(move |i| {
        app::handle_main(events::HIDE_ADD_FEED_WINDOW).unwrap();
        s_1.send(i.value());
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

    let s_2 = s.clone();
    ok_button.set_callback(move |_| {
        app::handle_main(events::HIDE_ADD_FEED_WINDOW).unwrap();
        s_2.send(input.value());
        input.set_value("");
    });

    buttons_pack.end();
    buttons_pack.resizable(&resizable_box);

    window.end();
    window
}

/// Create a Menu Bar (supposed to be a child of the Feeds pane)
pub fn menubar(menubar_height: i32) -> MenuBar {
    let mut feeds_menubar = MenuBar::default().with_size(0, menubar_height);
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

/// Create a Feeds Tree (supposed to be a child of the Feeds pane)
pub fn tree(window: &Window, menubar_height: i32) -> Tree {
    let mut feeds_tree = Tree::default().with_size(0, window.height() - menubar_height);
    feeds_tree.set_frame(FrameType::BorderBox);
    feeds_tree.set_show_root(false);
    feeds_tree.end();
    feeds_tree
}
