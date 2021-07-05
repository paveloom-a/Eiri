use crossbeam_channel::{self, Receiver, Sender};
use fltk::{
    app::{self, App},
    enums::FrameType,
    tree::TreeItem,
};

/// A struct providing access to the application's options
pub struct Options {
    pub window_min_width: i32,
    pub window_min_height: i32,
    pub feeds_width: i32,
    pub menubar_height: i32,
    pub vertical_border_width: i32,
    pub horizontal_border_height: i32,
}

impl Options {
    /// Get the default constant set of the application's options
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

/// Default constant set of the application's options
pub const OPTIONS: Options = Options::default();

macro_rules! channels_default_impl {
    (
        #[$struct_doc:meta]
        pub struct $name:ident {
        $(
            #[$field_doc:meta]
            pub $field_name:ident: $field_type:ty,
        )*
    }) => {
        #[$struct_doc]
        pub struct $name {
            $(
                #[$field_doc]
                pub $field_name: $field_type,
            )*
        }

        impl $name {
            /// Get the default set of the application's channels
            pub fn default() -> $name {
                $name {
                    $(
                        $field_name: Channel::new(crossbeam_channel::unbounded()),
                    )*
                }
            }
        }
    }
}

pub struct Channel<T> {
    pub s: Sender<T>,
    pub r: Receiver<T>,
}

impl<T> Channel<T> {
    fn new((s, r): (Sender<T>, Receiver<T>)) -> Channel<T> {
        Channel { s, r }
    }
}

channels_default_impl! {
    /// A struct providing access to the application's channels
    pub struct Channels {
        /// Channel 1: From Any Window to Main Window
        pub mw: Channel<i32>,
        /// Channel 2: From Main Window to Add Feed Window
        pub add_feed_window: Channel<i32>,
        /// Channel 3: From Main Window to Add Folder Window
        pub add_folder_window: Channel<i32>,
        /// Channel 4: From Feed Window's Input Widget / OK Button to Feeds Tree
        pub add_feed: Channel<String>,
        /// Channel 5: From Add Folder Window's Input Widget / OK Button to Feeds Tree
        pub add_folder: Channel<String>,
        /// Channel 6: From Feeds Tree to Add Folder's Location
        pub add_folder_location: Channel<Vec<TreeItem>>,
    }
}

/// Create a new App
pub fn new() -> App {
    let app = App::default();
    app::background(255, 255, 255);
    app::set_frame_type(FrameType::BorderBox);
    app
}
