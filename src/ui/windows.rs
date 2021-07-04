use fltk::{
    app,
    button::Button,
    enums::{Align, CallbackTrigger, FrameType},
    frame::Frame,
    group::{Pack, PackType, Tile},
    image::PngImage,
    input::Input,
    prelude::*,
    tree::Tree,
    window::{DoubleWindow, Window},
};

use super::app::{Channels, OPTIONS};
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
        OPTIONS.window_min_width,
        OPTIONS.window_min_height,
        "Eiri",
    );
    window.set_icon(Some(window_icon.clone()));
    window.size_range(OPTIONS.window_min_width, OPTIONS.window_min_height, 0, 0);
    window.make_resizable(true);
    window
}

/// Create the Window Tile (a child of the Main Window)
pub fn tile(window: &Window) -> Tile {
    let window_tile = Tile::default().with_size(window.width(), window.height());
    let mut window_tile_resize_box = Frame::default()
        .with_pos(
            window_tile.x() + OPTIONS.feeds_width + OPTIONS.vertical_border_width,
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
        let s = channels.add_feed_input_sender.clone();
        move |i, ev| match ev.bits() {
            events::ADD_FEED_WINDOW_SEND_INPUT => {
                s.try_send(i.value()).ok();
                true
            }
            events::ADD_FEED_WINDOW_CLEAR_INPUT => {
                i.set_value("");
                true
            }
            _ => false,
        }
    });

    input.set_callback({
        let s = channels.mw_signal_sender.clone();
        move |i| {
            app::handle_main(events::ADD_FEED_WINDOW_SEND_INPUT).ok();
            app::handle_main(events::HIDE_ADD_FEED_WINDOW).ok();
            i.set_value("");
            s.try_send(events::ADD_FEED_EVENT).ok();
        }
    });

    ok_button.set_callback({
        let s = channels.mw_signal_sender.clone();
        move |_| {
            app::handle_main(events::ADD_FEED_WINDOW_SEND_INPUT).ok();
            app::handle_main(events::ADD_FEED_WINDOW_CLEAR_INPUT).ok();
            app::handle_main(events::HIDE_ADD_FEED_WINDOW).ok();
            s.try_send(events::ADD_FEED_EVENT).ok();
        }
    });

    cancel_button.set_callback(|_| {
        app::handle_main(events::ADD_FEED_WINDOW_CLEAR_INPUT).ok();
        app::handle_main(events::HIDE_ADD_FEED_WINDOW).ok();
    });

    window
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
        let s = channels.add_folder_input_sender.clone();
        move |i, ev| match ev.bits() {
            events::ADD_FOLDER_WINDOW_SEND_INPUT => {
                s.try_send(i.value()).ok();
                true
            }
            events::ADD_FOLDER_WINDOW_CLEAR_INPUT => {
                i.set_value("");
                true
            }
            _ => false,
        }
    });

    input.set_callback({
        let s = channels.mw_signal_sender.clone();
        move |i| {
            app::handle_main(events::ADD_FOLDER_WINDOW_SEND_INPUT).ok();
            app::handle_main(events::HIDE_ADD_FOLDER_WINDOW).ok();
            i.set_value("");
            s.try_send(events::ADD_FOLDER_EVENT).ok();
        }
    });

    ok_button.set_callback({
        let s = channels.mw_signal_sender.clone();
        move |_| {
            app::handle_main(events::ADD_FOLDER_WINDOW_SEND_INPUT).ok();
            app::handle_main(events::ADD_FOLDER_WINDOW_CLEAR_INPUT).ok();
            app::handle_main(events::HIDE_ADD_FOLDER_WINDOW).ok();
            s.try_send(events::ADD_FOLDER_EVENT).ok();
        }
    });

    cancel_button.set_callback(|_| {
        app::handle_main(events::ADD_FOLDER_WINDOW_CLEAR_INPUT).ok();
        app::handle_main(events::HIDE_ADD_FOLDER_WINDOW).ok();
    });

    window
}
