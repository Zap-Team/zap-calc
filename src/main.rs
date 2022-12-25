mod application;
mod constants;
mod scene;
mod styles;
mod utils;

use iced::{Application, Settings};
use application::ZapCalc;

fn main() -> iced::Result {
    ZapCalc::run(Settings::default())
}
