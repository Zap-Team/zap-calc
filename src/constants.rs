use iced::Padding;

/// Enum with events emitting
/// while program running.
#[derive(Debug, Clone)]
pub enum Message {
    QueryUpdated(String),
    ButtonClicked,
    CIButtonClicked
}

/// Enum with two types of buttons:
/// * [`ButtonType::Standard`] - for standard buttons.
/// * [`ButtonType::CoreImportant`] - for special set of buttons.
pub enum ButtonType {
    Standard,
    CoreImportant
}

/// List with a lists containing string
/// sources of building buttons.
pub const BUTTONS_SOURCE: [[&'static str; 5]; 4] = [
    ["7", "8", "9", "+", "π"],
    ["4", "5", "6", "-", "²"],
    ["1", "2", "3", "*", "√"],
    ["0", ".", "%", "/", "="]
];

/// Padding accepted for all widget on the screen.
pub const DEFAULT_PADDING: Padding = Padding::new(1);
