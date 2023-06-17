use iced::executor;
use iced::widget::{button, column, container};
use iced::window::close;
use iced::{Alignment, Application, window, Command, Element, Length, Theme};

pub struct  Window {
  
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SinglePlayerButtonPressed,
    MultiplayerButtonPressed,
    SettingsButtonPressed,
    ExitButtonPressed,
}

impl Application for Window {
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
       //TODO create a constructor
       (Self {}, window::change_mode(iced::window::Mode::Fullscreen))
    }

    fn title(&self) -> String {
        String::from("3m")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SinglePlayerButtonPressed => {
                //TODO create event for Single Player button
            }
            Message::MultiplayerButtonPressed => {
                //TODO create event for Multiplayer button
            }
            Message::SettingsButtonPressed => {
                //TODO create event for Settings button
            }
            Message::ExitButtonPressed => {
                //TODO create event for Exit button
                return close();
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            button("Single player").on_press(Message::SinglePlayerButtonPressed),
            button("Multiplayer").on_press(Message::MultiplayerButtonPressed),
            button("Settings").on_press(Message::SettingsButtonPressed),
            button("Exit").on_press(Message::ExitButtonPressed),
        ]
        .padding(20)
        .spacing(20)
        .align_items(Alignment::Center);

        container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    
}
