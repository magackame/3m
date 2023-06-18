use iced::executor;
use iced::widget::{button, column, container};
use iced::{window, Alignment, Application, Command, Element, Length, Theme};

enum Page {
    StartMenu,
    SinglePlayer,
    Multiplayer,
    Game,
    Settings,
}

#[derive(Debug, Clone,Copy)]
enum StartMenuMessage {
    SinglePlayerButtonPressed,
    MultiplayerButtonPressed,
    SettingsButtonPressed,
    ExitButtonPressed,
}

#[derive(Debug, Clone,Copy)]
enum SinglePlayerMessage {
    //TODO
}

#[derive(Debug, Clone,Copy)]
enum MultiplayerMessage {
    //TODO
}

#[derive(Debug, Clone,Copy)]
enum GameMessage {
    //TODO
}

#[derive(Debug, Clone,Copy)]
enum SettingsMessage {
    //TODO
}

pub struct Window {
    current_page: Page,
}
#[derive(Debug, Clone,Copy)]
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
            // Block list events for buttons in start-menu
            Message::StartMenu(StartMenuMessage::SinglePlayerButtonPressed) => {
                self.current_page = Page::SinglePlayer;

                Command::none()
            }
            Message::StartMenu(StartMenuMessage::MultiplayerButtonPressed) => {
                self.current_page = Page::Multiplayer;

                Command::none()
            }
            Message::StartMenu(StartMenuMessage::SettingsButtonPressed) => {
                self.current_page = Page::Settings;

                Command::none()
            }
            Message::StartMenu(StartMenuMessage::ExitButtonPressed) => window::close(),

            //Next blocks
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
            Page::SinglePlayer => column![button("TestButton"),].into(),
            Page::Multiplayer => column![button("TestButton"),].into(),
            Page::Game => column![button("TestButton"),].into(),
            Page::Settings => column![button("TestButton"),].into(),
        }
    }
}
