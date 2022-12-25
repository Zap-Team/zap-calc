use crate::styles;
use crate::constants::{BUTTONS_SOURCE, ButtonType, Message};
use crate::utils::{ApplicationObjects, create_button};

use iced::Element;
use iced::widget;

/// Generates line from passed list with strings.
fn create_line(source: [&'static str; 5]) -> Element<Message> {
    let mut line = Vec::new();

    for string in source {
        let button_type = match string {
            "=" => ButtonType::CoreImportant,
            _ => ButtonType::Standard
        };

        let button = create_button(string, button_type);
        line.push(styles::style_button(button));
    }

    let row = widget::row(line.into());
    styles::style_row(row)
}

/// Generates widget with buttons for application.
fn get_keypad(_objects: &ApplicationObjects) -> Element<Message> {
    let mut lines = Vec::new();

    for line_source in BUTTONS_SOURCE {
        let line = create_line(line_source);
        lines.push(line);
    }

    let column = widget::column(lines.into());
    styles::style_column(column)
}

/// Generates new edit line for application.
fn get_query(objects: &ApplicationObjects) -> Element<Message> {
    let query = widget::text_input(
        "Expression:",
        objects.query.as_str(),
        Message::QueryUpdated
    );

    styles::style_text_input(query)
}

/// Generates the main scene for application.
pub fn get_scene(objects: &ApplicationObjects) -> widget::Column<Message> {
    let query = get_query(objects);
    let keypad = get_keypad(objects);

    let layout = vec![
        query.into(),
        keypad.into()
    ];

    widget::column(layout)
}
