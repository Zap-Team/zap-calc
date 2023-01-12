use crate::application::scene::get_scene;
use crate::application::utils;
use crate::constants::Message;

use iced::executor::Default;
use iced::{Application, Command, Element, Theme};

/// Main structure for running application.
pub struct ZapCalc {
    objects: utils::ApplicationObjects
}

impl Application for ZapCalc {
    type Executor = Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (ZapCalc, Command<Self::Message>) {
        (
            ZapCalc { objects: utils::ApplicationObjects::new() },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Zap Calc")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match _message {
            Message::QueryUpdated(message) => {
                self.objects.query = message;
            },
            Message::Execute => {
                println!("Must be executed.");
            },
            _ => {
                let label = utils::get_literal_equivalent(_message);
                self.objects.query.push_str(label.as_str());
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        get_scene(&self.objects)
            .into()
    }
}
