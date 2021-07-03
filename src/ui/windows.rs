use crossbeam_channel::{self, Receiver};
use fltk::{
    app,
    button::Button,
    enums::{Align, CallbackTrigger, FrameType},
    frame::Frame,
    group::{Pack, PackType, Tile},
    image::PngImage,
    input::Input,
    prelude::*,
    window::{DoubleWindow, Window},
};

use super::app::Options;
use crate::events;

/// Load a Window Icon
pub fn icon() -> PngImage {
    PngImage::from_data(include_bytes!("../../assets/eiri-32.png")).unwrap()
}

/// Create the Main Window
pub fn main(window_icon: &PngImage, options: &Options) -> DoubleWindow {
    let mut window = Window::new(
        100,
        100,
        options.window_min_width,
        options.window_min_height,
        "Eiri",
    );
    window.set_icon(Some(window_icon.clone()));
    window.size_range(options.window_min_width, options.window_min_height, 0, 0);
    window.make_resizable(true);
    window
}

/// Create the Window Tile (a child of the Main Window)
pub fn tile(window: &Window, options: &Options) -> Tile {
    let window_tile = Tile::default().with_size(window.width(), window.height());
    let mut window_tile_resize_box = Frame::default()
        .with_pos(
            window_tile.x() + options.feeds_width + options.vertical_border_width,
            window_tile.y(),
        )
        .with_size(window_tile.w() - 800, window_tile.h());
    window_tile_resize_box.hide();
    window_tile.resizable(&window_tile_resize_box);
    window_tile
}

/// Create the Add Feed Window (open using the Add Feed Button in the Menu Bar)
pub fn add_feed(window_icon: &PngImage) -> (Window, Receiver<String>) {
    // Channel 1: Feeds Tree and the Add Feed Window's Input Widget / OK Button
    let (s_1, r) = crossbeam_channel::unbounded::<String>();
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

    // 1.1 Input
    let mut input = Input::new(20, 50, window.width() - 40, 25, "Feed URL:");
    input.set_align(Align::TopLeft);
    input.set_frame(FrameType::BorderBox);
    input.set_trigger(CallbackTrigger::EnterKeyAlways);
    window.resizable(&input);

    input.set_callback(move |i| {
        s_1.send(i.value()).ok();
        app::handle_main(events::HIDE_ADD_FEED_WINDOW).ok();
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

    ok_button.visible_focus(false);
    ok_button.set_callback(move |_| {
        s_2.send(input.value()).ok();
        app::handle_main(events::HIDE_ADD_FEED_WINDOW).ok();
        input.set_value("");
    });

    buttons_pack.end();
    buttons_pack.resizable(&resizable_box);

    window.end();
    (window, r)
}
