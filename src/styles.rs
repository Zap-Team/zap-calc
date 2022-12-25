use crate::constants::{Message, DEFAULT_PADDING};

use iced::{Alignment, Element, Length, widget};

/// Styles passed [`TextInput`] with default application style.
pub fn style_text_input(text_input: widget::TextInput<Message>) -> Element<Message> {
    text_input.padding(DEFAULT_PADDING)
        // .width(Length::Fill) // FIXME: This not affects to widget width.
        .into()
}

/// Styles passed [`BUTTON`] with default application style.
pub fn style_button(button: widget::Button<Message>) -> Element<Message> {
    button.padding(DEFAULT_PADDING)
        .width(Length::Fill)
        .into()
}

/// Styles passed [`Row`] with default application style.
pub fn style_row(row: widget::Row<Message>) -> Element<Message> {
    row.align_items(Alignment::Center)
        .padding(DEFAULT_PADDING)
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

/// Styles passed [`Column`] with default application style.
pub fn style_column(column: widget::Column<Message>) -> Element<Message> {
    column.align_items(Alignment::Center)
        .padding(DEFAULT_PADDING)
        .width(Length::Fill)
        .into()
}
