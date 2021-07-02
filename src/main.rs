// // Switch from the console subsystem to the windows subsystem in the release mode
// #![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

// mod events;
// mod ui;

// use fltk::{app, prelude::*};

// fn main() {
//     // The app's Options
//     const OPTIONS: ui::app::Options = ui::app::Options {
//         window_min_width: 1000,
//         window_min_height: 600,
//         feeds_width: 200,
//         menubar_height: 30,
//         vertical_border_width: 2,
//         horizontal_border_height: 4,
//     };

//     // Override the system's screen scaling
//     for i in 0..app::screen_count() {
//         app::set_screen_scale(i, 1.0);
//     }

//     // 0. App
//     let app = ui::app::new();

//     // 1. Window
//     let window_icon = ui::window::icon();
//     let mut window = ui::window::new(&window_icon, &OPTIONS);

//     // 2. Window Tile
//     let window_tile = ui::window::tile(&window, &OPTIONS);

//     // 2.1 Feeds Pane
//     let feeds = ui::feeds::new(&window, &OPTIONS);

//     // 2.1.1 Feeds Pack
//     let feeds_pack = ui::feeds::pack(&OPTIONS);

//     // 2.1.1.1 Feeds MenuBar
//     let feeds_menubar = ui::feeds::menubar(&OPTIONS);

//     // 2.1.1.2 Feeds' Horizontal Border
//     let _feeds_horizontal_border = ui::feeds::horizontal_border(&OPTIONS);

//     // 2.1.1.3 Feeds Tree
//     let mut feeds_tree = ui::feeds::tree(&window, &OPTIONS);

//     feeds_pack.resizable(&feeds_menubar);
//     feeds_pack.resizable(&feeds_tree);
//     feeds_pack.end();

//     // 2.1.2 Feeds' Vertical Border
//     let _feeds_vertical_border = ui::feeds::vertical_border(&OPTIONS);

//     feeds.resizable(&feeds_pack);
//     feeds.end();

//     // 2.2 News Pane
//     let news = ui::news::new(&window, &OPTIONS);

//     // 2.2.1 News' Vertical Border
//     let _news_vertical_border = ui::news::vertical_border(&OPTIONS);

//     // 2.2.2 News Pack
//     let news_pack = ui::news::pack(&window, &OPTIONS);

//     // 2.2.2.1 News MenuBar
//     let news_menubar = ui::news::menubar(&OPTIONS);

//     // 2.2.2.2 News' Horizontal Border
//     let _news_horizontal_border = ui::news::horizontal_border(&OPTIONS);

//     // 2.2.2.3 News Feed
//     let news_feed = ui::news::feed(&window, &OPTIONS);

//     news_pack.resizable(&news_menubar);
//     news_pack.resizable(&news_feed);
//     news_pack.end();

//     news.resizable(&news_pack);
//     news.end();

//     window_tile.end();

//     window.end();
//     window.show();

//     // Hidden windows

//     // 1. Add Feed Window
//     let (add_feed_window, r_ch1) = ui::feeds::add_feed_window(&window_icon);

//     // Redirect the signals to other windows
//     window.handle(move |_, ev| {
//         let mut rv: bool = false;
//         let ev = ev.bits();
//         if ev == events::SHOW_ADD_FEED_WINDOW {
//             app::handle(ev, &add_feed_window).unwrap();
//             rv = true;
//         }
//         rv
//     });

//     // Start the event loop
//     while app.wait() {
//         if let Some(path) = r_ch1.recv() {
//             println!("Received the path: {}", path);
//             feeds_tree.add(path.as_str());
//             feeds_tree.redraw();
//         }
//     }
// }

use fltk::{
    app::{self, App},
    button::Button,
    enums::CallbackTrigger,
    input::Input,
    prelude::*,
    window::Window,
};

fn main() {
    let app = App::default();

    let mut main_window = Window::new(100, 100, 400, 300, "Main Window");
    main_window.end();
    main_window.show();

    let mut second_window = Window::new(200, 200, 400, 300, "Second Window");
    let mut input = Input::new(160, 100, 80, 40, "");
    let mut button = Button::new(160, 150, 60, 40, "Send!");
    second_window.end();
    second_window.show();

    let (s_1, r) = app::channel::<String>();
    let s_2 = s_1.clone();

    input.set_trigger(CallbackTrigger::EnterKeyAlways);
    input.set_callback(move |i| {
        s_1.send(i.value());
    });

    button.set_callback(move |_| {
        s_2.send(input.value());
    });

    while app.wait() {
        if let Some(msg) = r.recv() {
            println!("Got {}.", msg);
        }
    }
}
