use std::fmt::Display;

use crate::str;
use crate::constants::{Message, ButtonType};

use iced::widget;

/// Creates new button with binding its click with specified set of
/// buttons ([`Message::ButtonClicked`] or [`Message::CIButtonClicked`]).
pub fn create_button(content: &'static str, with_type: ButtonType) -> widget::Button<Message> {
    let empty_button = widget::button(content);

    match with_type {
        ButtonType::Standard => {
            empty_button.on_press(Message::ButtonClicked)
        },
        ButtonType::CoreImportant => {
            empty_button.on_press(Message::CIButtonClicked)
        }
    }
}

/// Creates new button from **ANY** type content with binding its click
/// with specified set of buttons ([`Message::ButtonClicked`] or
/// [`Message::CIButtonClicked`]).
// TODO: later `#[deprecated]` must be removed.
#[deprecated]
#[allow(dead_code)]
pub fn create_button_from<T>(content: T, with_type: ButtonType) -> widget::Button<'static, Message>
where
    T: Display
{
    let string_content = str!(content);
    create_button(string_content, with_type)
}

/// Structure for storing some variables
/// during program running.
pub struct ApplicationObjects {
    pub query: String
}

impl ApplicationObjects {
    /// Creates new empty structure.
    /// Mostly used one time.
    pub fn new() -> ApplicationObjects {
        ApplicationObjects { query: String::new() }
    }
}

/// Creates `String` object from
/// passed object of ANY type.
///
/// Based on a [`format`] macro.
#[macro_export]
macro_rules! string {
    ($val:expr) => {
        format!("{}", $val)
    };
}

/// Creates `&str` object from
/// passed object of ANY type.
///
/// Based on a [`string`] macro.
#[macro_export]
macro_rules! str {
    ($val:expr) => {
        Box::leak(
            $crate::string!($val)
                .into_boxed_str()
        )
    };
}
