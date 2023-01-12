use crate::constants;

use iced::{Alignment, Element, Length, widget};

/// Styles passed [`TextInput`] with default application style.
pub fn style_text_input(
    text_input: widget::TextInput<constants::Message>
) -> Element<constants::Message> {
    text_input.padding(constants::DEFAULT_PADDING)
        // .width(Length::Fill) // FIXME: This not affects to widget width.
        .into()
}

/// Styles passed [`BUTTON`] with default application style.
pub fn style_button(
    button: widget::Button<constants::Message>
) -> Element<constants::Message> {
    button.padding(constants::DEFAULT_PADDING)
        .width(Length::Fill)
        .into()
}

/// Styles passed [`Row`] with default application style.
pub fn style_row(
    row: widget::Row<constants::Message>
) -> Element<constants::Message> {
    row.align_items(Alignment::Center)
        .padding(constants::DEFAULT_PADDING)
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

/// Styles passed [`Column`] with default application style.
pub fn style_column(
    column: widget::Column<constants::Message>
) -> Element<constants::Message> {
    column.align_items(Alignment::Center)
        .padding(constants::DEFAULT_PADDING)
        .width(Length::Fill)
        .into()
}
