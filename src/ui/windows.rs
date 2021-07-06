use fltk::{
    app,
    button::Button,
    enums::{Align, CallbackTrigger, Event, FrameType},
    frame::Frame,
    group::{Pack, PackType, Tile},
    image::PngImage,
    input::Input,
    prelude::*,
    tree::{Tree, TreeItem},
    window::{DoubleWindow, Window},
};

use super::app::CONSTANTS;
use crate::channels::Channels;
use crate::events;

/// Load a Window Icon
pub fn icon() -> PngImage {
    PngImage::from_data(include_bytes!("../../assets/eiri-32.png")).unwrap()
}

/// Create the Main Window
pub fn main(window_icon: &PngImage) -> DoubleWindow {
    let mut window = Window::new(
        100,
        100,
        CONSTANTS.window_min_width,
        CONSTANTS.window_min_height,
        "Eiri",
    );
    window.set_icon(Some(window_icon.clone()));
    window.size_range(
        CONSTANTS.window_min_width,
        CONSTANTS.window_min_height,
        0,
        0,
    );
    window.make_resizable(true);
    window
}

/// Create the Window Tile (a child of the Main Window)
pub fn tile(window: &Window) -> Tile {
    let window_tile = Tile::default().with_size(window.width(), window.height());
    let mut window_tile_resize_box = Frame::default()
        .with_pos(
            window_tile.x() + CONSTANTS.feeds_width + CONSTANTS.vertical_border_width,
            window_tile.y(),
        )
        .with_size(window_tile.w() - 800, window_tile.h());
    window_tile_resize_box.hide();
    window_tile.resizable(&window_tile_resize_box);
    window_tile
}

/// Create the Add Feed Window (open using the Add Feed Button in the Menu Bar)
pub fn add_feed(window_icon: &PngImage, channels: &Channels) -> Window {
    // 1. Window
    let mut window = Window::default()
        .with_size(300, 200)
        .center_screen()
        .with_label("Add feed");
    window.size_range(300, 200, 0, 200);
    window.set_icon(Some(window_icon.clone()));
    window.make_resizable(true);
    window.make_modal(true);

    // 1.1 Input
    let mut input = Input::new(20, 50, window.width() - 40, 25, "Feed URL:");
    input.set_align(Align::TopLeft);
    input.set_frame(FrameType::BorderBox);
    input.set_trigger(CallbackTrigger::EnterKeyAlways);

    // 1.2 Buttons' Pack
    let mut buttons_pack = Pack::default()
        .with_pos(window.width() - 2 * 80 - 2 * 10 - 25, window.height() - 40)
        .with_size(85, 25);
    buttons_pack.set_type(PackType::Horizontal);
    buttons_pack.set_spacing(10);

    // 1.2.1 Resizable Box
    let resizable_box = Frame::default().with_size(5, 0);

    // 1.2.2 OK Button
    let mut ok_button = Button::default().with_size(80, 0).with_label("OK");
    ok_button.visible_focus(false);

    // 1.2.3 Cancel Button
    let mut cancel_button = Button::default().with_size(80, 0).with_label("Cancel");
    cancel_button.visible_focus(false);

    buttons_pack.end();
    buttons_pack.resizable(&resizable_box);

    window.resizable(&input);
    window.end();

    add_feed_logic(
        channels,
        &mut window,
        &mut input,
        &mut ok_button,
        &mut cancel_button,
    );

    window
}

/// Add logic to the widgets of the Add Feed Window
pub fn add_feed_logic(
    channels: &Channels,
    window: &mut Window,
    input: &mut Input,
    ok_button: &mut Button,
    cancel_button: &mut Button,
) {
    window.handle(move |w, ev| match ev.bits() {
        events::SHOW_ADD_FEED_WINDOW => {
            w.show();
            true
        }
        events::HIDE_ADD_FEED_WINDOW => {
            w.hide();
            true
        }
        _ => false,
    });

    input.handle({
        let s = channels.add_feed.s.clone();
        move |i, ev| match ev {
            Event::Hide => {
                i.set_value("");
                true
            }
            _ => match ev.bits() {
                events::ADD_FEED_WINDOW_SEND_INPUT => {
                    s.try_send(i.value()).ok();
                    true
                }
                _ => false,
            },
        }
    });

    input.set_callback({
        let s = channels.mw.s.clone();
        move |_| {
            app::handle_main(events::ADD_FEED_WINDOW_SEND_INPUT).ok();
            app::handle_main(events::HIDE_ADD_FEED_WINDOW).ok();
            s.try_send(events::ADD_FEED_EVENT).ok();
        }
    });

    ok_button.set_callback({
        let s = channels.mw.s.clone();
        move |_| {
            app::handle_main(events::ADD_FEED_WINDOW_SEND_INPUT).ok();
            app::handle_main(events::HIDE_ADD_FEED_WINDOW).ok();
            s.try_send(events::ADD_FEED_EVENT).ok();
        }
    });

    cancel_button.set_callback(|_| {
        app::handle_main(events::HIDE_ADD_FEED_WINDOW).ok();
    });
}

/// Create the Add Folder Window (open using the Add Folder Button in the Menu Bar)
pub fn add_folder(window_icon: &PngImage, channels: &Channels) -> Window {
    // 1. Window
    let mut window = Window::default()
        .with_size(400, 330)
        .center_screen()
        .with_label("Add folder");
    window.size_range(400, 330, 0, 330);
    window.set_icon(Some(window_icon.clone()));
    window.make_resizable(true);
    window.make_modal(true);

    // 1.1 Input
    let mut input = Input::new(20, 30, window.width() - 40, 25, "Name:");
    input.set_align(Align::TopLeft);
    input.set_frame(FrameType::BorderBox);
    input.set_trigger(CallbackTrigger::EnterKeyAlways);

    // 1.2 Location Tree
    let mut location = Tree::new(
        20,
        80,
        window.width() - 40,
        window.height() - 140,
        "Location:",
    );
    location.set_align(Align::TopLeft);
    location.set_frame(FrameType::BorderBox);
    location.set_root_label("All Feeds");

    // 1.3 Buttons' Pack
    let mut buttons_pack = Pack::default()
        .with_pos(window.width() - 2 * 80 - 2 * 10 - 25, window.height() - 40)
        .with_size(5 + 2 * 80, 25);
    buttons_pack.set_type(PackType::Horizontal);
    buttons_pack.set_spacing(10);

    // 1.2.1 Resizable Box
    let resizable_box = Frame::default().with_size(5, 0);

    // 1.2.2 OK Button
    let mut ok_button = Button::default().with_size(80, 0).with_label("OK");
    ok_button.visible_focus(false);

    // 1.2.3 Cancel Button
    let mut cancel_button = Button::default().with_size(80, 0).with_label("Cancel");
    cancel_button.visible_focus(false);

    buttons_pack.end();
    buttons_pack.resizable(&resizable_box);

    window.resizable(&input);
    window.resizable(&location);
    window.end();

    add_folder_logic(
        channels,
        &mut window,
        &mut input,
        &mut location,
        &mut ok_button,
        &mut cancel_button,
    );

    window
}

/// Add logic to the widgets of the Add Folder Window
pub fn add_folder_logic(
    channels: &Channels,
    window: &mut Window,
    input: &mut Input,
    location: &mut Tree,
    ok_button: &mut Button,
    cancel_button: &mut Button,
) {
    window.handle(move |w, ev| match ev.bits() {
        events::SHOW_ADD_FOLDER_WINDOW => {
            w.show();
            true
        }
        events::HIDE_ADD_FOLDER_WINDOW => {
            w.hide();
            true
        }
        _ => false,
    });

    input.handle({
        let r_path = channels.add_folder_input.r.clone();
        let s_input = channels.add_folder.s.clone();
        move |i, ev| match ev {
            Event::Hide => {
                i.set_value("");
                true
            }
            _ => match ev.bits() {
                events::ADD_FOLDER_WINDOW_SEND_INPUT => {
                    if let Ok(path) = r_path.try_recv() {
                        s_input.try_send(path + "/" + &i.value()).ok();
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
        }
    });

    input.set_callback({
        let s = channels.mw.s.clone();
        move |_| {
            app::handle_main(events::ADD_FOLDER_WINDOW_SEND_LOCATION).ok();
            app::handle_main(events::ADD_FOLDER_WINDOW_SEND_INPUT).ok();
            app::handle_main(events::HIDE_ADD_FOLDER_WINDOW).ok();
            s.try_send(events::ADD_FOLDER_EVENT).ok();
        }
    });

    location.handle({
        let s_folder = channels.add_folder_input.s.clone();
        let r_feeds_tree = channels.add_folder_location.r.clone();
        move |l, ev| match ev {
            Event::Hide => l.root().map_or(false, |root| {
                if let Some(item) = l.first_selected_item() {
                    l.select_toggle(&item, false)
                }
                l.clear_children(&root);
                true
            }),
            _ => match ev.bits() {
                events::ADD_FOLDER_WINDOW_LOAD_LOCATION => {
                    if let Ok(items) = r_feeds_tree.try_recv() {
                        for mut item in items {
                            l.add(&get_item_path(&mut item));
                        }
                        l.redraw();
                        true
                    } else {
                        false
                    }
                }
                events::ADD_FOLDER_WINDOW_SEND_LOCATION => l.first_selected_item().map_or_else(
                    || s_folder.try_send(String::default()).is_ok(),
                    |mut item| s_folder.try_send(get_item_path(&mut item)).is_ok(),
                ),
                _ => false,
            },
        }
    });

    ok_button.set_callback({
        let s = channels.mw.s.clone();
        move |_| {
            app::handle_main(events::ADD_FOLDER_WINDOW_SEND_LOCATION).ok();
            app::handle_main(events::ADD_FOLDER_WINDOW_SEND_INPUT).ok();
            app::handle_main(events::HIDE_ADD_FOLDER_WINDOW).ok();
            s.try_send(events::ADD_FOLDER_EVENT).ok();
        }
    });

    cancel_button.set_callback(|_| {
        app::handle_main(events::HIDE_ADD_FOLDER_WINDOW).ok();
    });
}

/// Get the path to the tree item
pub fn get_item_path(item: &mut TreeItem) -> String {
    let mut path = String::default();
    if let Some(label) = item.label() {
        path.push_str(&label);
    }
    while let Some(parent) = item.parent() {
        if let Some(label) = parent.label() {
            if label.is_empty() || label == "All Feeds" {
                break;
            }
            path = label + "/" + &path;
        }
        *item = parent;
    }
    if path == "All Feeds" {
        String::default()
    } else {
        path
    }
}
