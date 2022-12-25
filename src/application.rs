use crate::constants::Message;
use crate::scene::get_scene;
use crate::utils::ApplicationObjects;

use iced::executor::Default;
use iced::{Application, Command, Element, Theme};

/// Main structure for running application.
pub struct ZapCalc {
    objects: ApplicationObjects
}

impl Application for ZapCalc {
    type Executor = Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (ZapCalc, Command<Self::Message>) {
        (
            ZapCalc { objects: ApplicationObjects::new() },
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
            Message::ButtonClicked => {
                println!("Default button pressed.");
            },
            Message::CIButtonClicked => {
                println!("Core important button pressed.");
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        get_scene(&self.objects)
            .into()
    }
}
