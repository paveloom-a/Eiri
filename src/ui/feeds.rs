use std::cell::RefCell;
use std::rc::Rc;

use fltk::{
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

/// Create a Feeds pane (as a `Pack`)
pub fn new(window: &Window, feeds_width: i32) -> Pack {
    let mut feeds = Pack::default().with_size(feeds_width, window.height());
    feeds.set_frame(FrameType::BorderFrame);
    feeds.set_color(Color::Black);
    feeds
}

/// Create a `MenuBar` widget (supposed to be a child of the Feeds pane)
pub fn menubar(menubar_height: i32) -> MenuBar {
    let mut feeds_menubar = MenuBar::default().with_size(0, menubar_height);
    feeds_menubar.end();
    feeds_menubar
}

/// Create a `Tree` widget (supposed to be a child of the Feeds pane)
pub fn tree(window: &Window, menubar_height: i32) -> Tree {
    let mut feeds_tree = Tree::default().with_size(0, window.height() - menubar_height);
    feeds_tree.set_frame(FrameType::BorderBox);
    feeds_tree.set_show_root(false);
    feeds_tree.end();
    feeds_tree
}

/// Create buttons in the `MenuBar` to interact with the `Tree`.
pub fn menubar_buttons(feeds_menubar: &mut MenuBar, feeds_tree: Tree, window_icon: PngImage) {
    let feeds_tree = Rc::new(RefCell::new(feeds_tree));

    let add_feed_closure_1 = move || {
        let mut add_feed_window = Window::default()
            .with_size(300, 200)
            .center_screen()
            .with_label("Add feed");
        add_feed_window.size_range(300, 200, 0, 200);
        add_feed_window.set_icon(Some(window_icon.clone()));
        add_feed_window.make_resizable(true);
        add_feed_window.make_modal(true);

        let mut add_feed_window_input =
            Input::new(20, 50, add_feed_window.width() - 40, 25, "Feed URL:");
        add_feed_window_input.set_align(Align::TopLeft);
        add_feed_window_input.set_frame(FrameType::BorderBox);
        add_feed_window_input.set_trigger(CallbackTrigger::EnterKeyAlways);
        add_feed_window.resizable(&add_feed_window_input);

        let mut add_feed_window_buttons_pack = Pack::default()
            .with_pos(195, add_feed_window.height() - 40)
            .with_size(85, 25);
        add_feed_window_buttons_pack.set_type(PackType::Horizontal);

        let add_feed_window_buttons_resizable = Frame::default().with_size(5, 0);

        let mut add_feed_window_button = Button::default().with_size(80, 0).with_label("OK");

        add_feed_window_buttons_pack.end();
        add_feed_window_buttons_pack.resizable(&add_feed_window_buttons_resizable);

        add_feed_window.end();
        add_feed_window.show();

        let feeds_tree_1 = Rc::clone(&feeds_tree);
        let feeds_tree_2 = Rc::clone(&feeds_tree);

        let add_feed_window_1 = Rc::new(RefCell::new(add_feed_window));
        let add_feed_window_2 = Rc::clone(&add_feed_window_1);

        add_feed_window_input.set_callback(move |i| match feeds_tree_1.try_borrow_mut() {
            Ok(mut feeds_tree) => {
                feeds_tree.add(i.value().as_str());
                add_feed_window_1.borrow_mut().hide();
            }
            Err(_) => {
                add_feed_window_1.borrow_mut().hide();
            }
        });
        add_feed_window_button.set_callback(move |_| match feeds_tree_2.try_borrow_mut() {
            Ok(mut feeds_tree) => {
                feeds_tree.add(add_feed_window_input.value().as_str());
                add_feed_window_2.borrow_mut().hide();
            }
            Err(_) => {
                add_feed_window_2.borrow_mut().hide();
            }
        });
    };
    let add_feed_closure_2 = add_feed_closure_1.clone();

    feeds_menubar.add(
        "@#+/Add Feed...",
        Shortcut::from_char('a'),
        MenuFlag::Normal,
        move |_| add_feed_closure_1(),
    );

    feeds_menubar.add(
        "@#+/Add Folder...",
        Shortcut::from_char('f'),
        MenuFlag::Normal,
        move |_| add_feed_closure_2(),
    );
}
