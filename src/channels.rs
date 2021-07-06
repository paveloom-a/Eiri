use crossbeam_channel::{self, Receiver, Sender};
use fltk::tree::TreeItem;

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
        /// Channel 7: From Add Folder's Location to Add Folder's Input
        pub add_folder_input: Channel<String>,
    }
}
