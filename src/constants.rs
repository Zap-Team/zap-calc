use iced::Padding;

/// Enum with events emitting
/// while program running.
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    QueryUpdated(String),
    Execute,

    // Buttons:
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,

    // Operators:
    Add,
    Sub,
    Mul,
    Div,
    Percent,

    // Other characters:
    Pi,
    Square,
    Sqrt,
    Dot
}

/// To access enum members directly.
use Message::*;

/// List with a lists containing string
/// sources for building buttons.
pub const BUTTONS_SOURCE: [[Message; 5]; 4] = [
    [Seven, Eight, Nine   , Add, Pi     ],
    [Four , Five , Six    , Sub, Square ],
    [One  , Two  , Three  , Mul, Sqrt   ],
    [Zero , Dot  , Percent, Div, Execute]
];

/// Padding accepted for all widgets on the screen.
pub const DEFAULT_PADDING: Padding = Padding::new(1);

/// Characters must be replaced with another before
/// evaluating full queue.
pub const RESERVED_CHARS: [char; 3] = [
    'π', '²', '√'
];
