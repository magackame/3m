use iced::executor;
use iced::widget::{button, column, container};
use iced::window::close;
use iced::{window, Alignment, Application, Command, Element, Length, Theme};

enum Page {
    StartMenu,
    SinglePlayer,
    Multiplayer,
    Game,
    Settings,
}

#[derive(Debug, Clone)]
enum StartMenuMessage {
    SinglePlayerButtonPressed,
    MultiplayerButtonPressed,
    SettingsButtonPressed,
    ExitButtonPressed,
}

#[derive(Debug, Clone)]
enum SinglePlayerMessage {
    //TODO
}

#[derive(Debug, Clone)]
enum MultiplayerMessage {
    //TODO
}

#[derive(Debug, Clone)]
enum GameMessage {
    //TODO
}

#[derive(Debug, Clone)]
enum SettingsMessage {
    //TODO
}

pub struct Window {
    current_page: Page,
}
#[derive(Debug, Clone)]
pub enum Message {
    StartMenu(StartMenuMessage),
    SinglePlayer(SinglePlayerMessage),
    Multiplayer(MultiplayerMessage),
    Game(GameMessage),
    Settings(SettingsMessage),
}

impl Application for Window {
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        //TODO create a constructor
        (
            Self {
                current_page: Page::StartMenu,
            },
            window::change_mode(iced::window::Mode::Fullscreen),
        )
    }

    fn title(&self) -> String {
        String::from("3m")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            // Block
            Message::StartMenu(StartMenuMessage::SinglePlayerButtonPressed) => {
                //TODO create event for Single Player button
                Command::none()
            }
            Message::StartMenu(StartMenuMessage::MultiplayerButtonPressed) => {
                //TODO create event for Multiplayer button
                Command::none()
            }
            Message::StartMenu(StartMenuMessage::SettingsButtonPressed) => {
                //TODO create event for Settings button
                Command::none()
            }
            Message::StartMenu(StartMenuMessage::ExitButtonPressed) => window::close(),

            //Next block
            Message::SinglePlayer(_) => todo!(),
            Message::Multiplayer(_) => todo!(),
            Message::Game(_) => todo!(),
            Message::Settings(_) => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        match self.current_page {
            Page::StartMenu => {
                let content = column![
                    button("Single player").on_press(Message::StartMenu(
                        StartMenuMessage::SinglePlayerButtonPressed
                    )),
                    button("Multiplayer").on_press(Message::StartMenu(
                        StartMenuMessage::MultiplayerButtonPressed
                    )),
                    button("Settings")
                        .on_press(Message::StartMenu(StartMenuMessage::SettingsButtonPressed)),
                    button("Exit")
                        .on_press(Message::StartMenu(StartMenuMessage::ExitButtonPressed)),
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
            Page::SinglePlayer => todo!(),
            Page::Multiplayer => todo!(),
            Page::Game => todo!(),
            Page::Settings => todo!(),
        }
    }
}
