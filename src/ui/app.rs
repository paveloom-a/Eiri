use crossbeam_channel::{self, Receiver, Sender};
use fltk::{
    app::{self, App},
    enums::FrameType,
};

/// The app's Options
pub struct Options {
    pub window_min_width: i32,
    pub window_min_height: i32,
    pub feeds_width: i32,
    pub menubar_height: i32,
    pub vertical_border_width: i32,
    pub horizontal_border_height: i32,
}

impl Options {
    /// Create an Options struct with default values
    const fn default() -> Options {
        Options {
            window_min_width: 1000,
            window_min_height: 600,
            feeds_width: 200,
            menubar_height: 30,
            vertical_border_width: 2,
            horizontal_border_height: 4,
        }
    }
}

/// Default set of the application options
pub const OPTIONS: Options = Options::default();

pub struct Channels {
    // Channel 1: Any Window (Sender / Translator)
    pub mw_signal_sender: Sender<i32>,
    // Channel 1: Main Window (Receiver / Translated)
    pub mw_signal_receiver: Receiver<i32>,
    /// Channel 2: Add Feed Window (Sender / Translated)
    pub a_feed_w_signal: Sender<i32>,
    /// Channel 2: Main Window (Receiver / Translator)
    pub mw_a_feed_w_translator: Receiver<i32>,
    /// Channel 3: Add Folder Window (Sender / Translated)
    pub a_folder_w_signal: Sender<i32>,
    /// Channel 3: Main Window (Receiver / Translator)
    pub mw_a_folder_w_translator: Receiver<i32>,
    /// Channel 4: Feed Window's Input Widget / OK Button (Sender)
    pub add_feed_input_sender: Sender<String>,
    /// Channel 4: Feeds Tree (Receiver)
    pub add_feed_input_receiver: Receiver<String>,
    /// Channel 5: Add Folder Window's Input Widget / OK Button (Sender)
    pub add_folder_input_sender: Sender<String>,
    /// Channel 5: Feeds Tree (Receiver)
    pub add_folder_input_receiver: Receiver<String>,
}

impl Channels {
    ///
    pub fn default() -> Channels {
        let (mw_signal_sender, mw_signal_receiver) = crossbeam_channel::unbounded();
        let (a_feed_w_signal, mw_a_feed_w_translator) = crossbeam_channel::unbounded();
        let (a_folder_w_signal, mw_a_folder_w_translator) = crossbeam_channel::unbounded();
        let (add_feed_input_sender, add_feed_input_receiver) = crossbeam_channel::unbounded();
        let (add_folder_input_sender, add_folder_input_receiver) = crossbeam_channel::unbounded();

        Channels {
            mw_signal_sender,
            mw_signal_receiver,
            a_feed_w_signal,
            mw_a_feed_w_translator,
            a_folder_w_signal,
            mw_a_folder_w_translator,
            add_feed_input_sender,
            add_feed_input_receiver,
            add_folder_input_sender,
            add_folder_input_receiver,
        }
    }
}

/// Create a new App
pub fn new() -> App {
    let app = App::default();
    app::background(255, 255, 255);
    app::set_frame_type(FrameType::BorderBox);
    app
}
