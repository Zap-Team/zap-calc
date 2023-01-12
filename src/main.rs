mod application;
mod constants;
mod evaluator;
mod macros;

use iced::{Application, Settings};
use application::ZapCalc;

fn main() -> iced::Result {
    ZapCalc::run(Settings::default())
}
