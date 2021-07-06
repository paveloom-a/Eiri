/// Consume the first argument in exchange of unit
macro_rules! unit {
    ($_t:tt $unit:expr) => {
        $unit
    };
}

/// Create events using provided identifiers
macro_rules! events {
    ( $first_event:ident $(,)? $($other_events:ident),* ) => {
            pub const $first_event: i32 = 1000 + <[()]>::len(&[$(unit!(($other_events) ())),*]) as i32;
            events!($($other_events),*);
    };
    () => {};
}

events!(
    SHOW_ADD_FEED_WINDOW,
    HIDE_ADD_FEED_WINDOW,
    ADD_FEED_WINDOW_SEND_INPUT,
    ADD_FEED_EVENT,
    SHOW_ADD_FOLDER_WINDOW,
    HIDE_ADD_FOLDER_WINDOW,
    ADD_FOLDER_WINDOW_SEND_INPUT,
    ADD_FOLDER_WINDOW_LOAD_LOCATION,
    ADD_FOLDER_WINDOW_SEND_LOCATION,
    ADD_FOLDER_EVENT
);
