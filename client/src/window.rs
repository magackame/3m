use iced::executor;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{window, Alignment, Application, Command, Element, Length, Theme};

enum Page {
    StartMenu,
    Lobby,
    Game,
    Settings,
}

#[derive(Debug, Clone)]
enum StartMenuMessage {
    StartGameButtonPressed,
    SettingsButtonPressed,
    ExitButtonPressed,
}

#[derive(Debug, Clone)]
enum LobbyMessage {
    CountPlayerInputChanged(String),
    StartGame,
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
    count_player: String,
    max_count_player: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    StartMenu(StartMenuMessage),
    Lobby(LobbyMessage),
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
                count_player: "".to_string(),
                max_count_player: 6,
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
            Message::StartMenu(StartMenuMessage::StartGameButtonPressed) => {
                self.current_page = Page::Lobby;

                Command::none()
            }
            Message::StartMenu(StartMenuMessage::SettingsButtonPressed) => {
                self.current_page = Page::Settings;

                Command::none()
            }
            Message::StartMenu(StartMenuMessage::ExitButtonPressed) => window::close(),

            //Next blocks
            Message::Lobby(LobbyMessage::CountPlayerInputChanged(new_value)) => {
                if new_value == "" {
                    self.count_player = "".to_string();
                } else {
                    match new_value.parse::<usize>() {
                        Ok(t) => {
                            self.count_player = if t <= self.max_count_player {
                                t.to_string()
                            } else {
                                self.count_player.to_string()
                            }
                        }
                        Err(_) => (),
                    }
                }

                Command::none()
            }
            Message::Lobby(LobbyMessage::StartGame) => Command::none(),

            Message::Game(_) => todo!(),
            Message::Settings(_) => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        match self.current_page {
            Page::StartMenu => {
                let content = column![
                    button("Start game")
                        .on_press(Message::StartMenu(StartMenuMessage::StartGameButtonPressed)),
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
            Page::Lobby => {
                let count_player_row = row![
                    text("Count players").size(20),
                    text_input("", &self.count_player).on_input(|count_player_value| {
                        Message::Lobby(LobbyMessage::CountPlayerInputChanged(count_player_value))
                    }),
                ]
                .padding(20)
                .spacing(20);

                let content = column![
                    count_player_row,
                    button("Save settings and start game")
                        .on_press(Message::Lobby(LobbyMessage::StartGame)),
                ]
                .padding(20)
                .spacing(20)
                .align_items(Alignment::Center);

                container(content).into()
            }
            Page::Game => column![button("GameTestButton"),].into(),
            Page::Settings => column![button("SettingsTestButton"),].into(),
        }
    }
}
