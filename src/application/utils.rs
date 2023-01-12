use crate::str;
use crate::constants::Message;

use iced::widget;

/// Returns an [`&str`] literal, that equals to passed message.
pub fn get_literal_equivalent(message: Message) -> String {
    use Message::*;

    let literal = match message {
        Execute => "=",
        One     => "1",
        Two     => "2",
        Three   => "3",
        Four    => "4",
        Five    => "5",
        Six     => "6",
        Seven   => "7",
        Eight   => "8",
        Nine    => "9",
        Zero    => "0",
        Add     => "+",
        Sub     => "-",
        Mul     => "*",
        Div     => "/",
        Percent => "%",
        Pi      => "π",
        Square  => "²",
        Sqrt    => "√",
        Dot     => ".",
        _       => " "
    };

    literal.to_string()
}

/// Creates new button with binding its click with specified set of
/// buttons ([`Message::ButtonClicked`] or [`Message::CIButtonClicked`]).
pub fn create_button(content: String, message: Message) -> widget::Button<'static, Message> {
    let str_content = str!(content);
    let empty_button = widget::button(str_content);
    empty_button.on_press(message)
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
