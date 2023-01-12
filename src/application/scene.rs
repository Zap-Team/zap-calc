use crate::application::utils;
use crate::application::styles;
use crate::constants;

use iced::{Element, widget};

/// Generates line from passed list with types.
fn create_line(
    line_source: [constants::Message; 5]
) -> Element<'static, constants::Message> {
    let mut line = Vec::new();

    for button_type in line_source {
        let label = utils::get_literal_equivalent(button_type.clone());
        let button = utils::create_button(label, button_type);

        line.push(styles::style_button(button));
    }

    let row = widget::row(line);
    styles::style_row(row)
}

/// Generates widget with buttons for application.
fn get_keypad(
    _objects: &utils::ApplicationObjects // TODO: Must be removed.
) -> Element<constants::Message> {
    let mut lines = Vec::new();

    for line_source in constants::BUTTONS_SOURCE {
        let line = create_line(line_source);
        lines.push(line);
    }

    let column = widget::column(lines);
    styles::style_column(column)
}

/// Generates new edit line for application.
fn get_query(
    objects: &utils::ApplicationObjects
) -> Element<constants::Message> {
    let query = widget::text_input(
        "Expression:",
        objects.query.as_str(),
        constants::Message::QueryUpdated
    );

    styles::style_text_input(query)
}

/// Generates the main scene for application.
pub fn get_scene(
    objects: &utils::ApplicationObjects
) -> widget::Column<constants::Message> {
    let query = get_query(objects);
    let keypad = get_keypad(objects);

    let layout = vec![query, keypad];
    widget::column(layout)
}
