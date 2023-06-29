use iced::executor;
use iced::widget::{button, checkbox, column, container, row, text, text_input};
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
    NicknamePlayerInputChanged(String),
    StartCapitalInputChanged(String),
    EnableBotCheckBox(bool),
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
    //Pages
    current_page: Page,

    //Lobby values
    count_player: String,
    max_count_player: usize,
    nick_player: String,
    enable_bot: bool,
    start_capital: String,
    max_start_capital: f64,
    //TODO values
}

#[derive(Debug, Clone)]
pub enum Message {
    StartMenu(StartMenuMessage),
    Lobby(LobbyMessage),
    Game(GameMessage),
    Settings(SettingsMessage),
}

impl Window {
    fn is_lobby_value_filled(&self) -> bool {
        if self.count_player == "" || self.nick_player == "" || self.start_capital == "" {
            return false;
        }

        true
    }
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
                count_player: String::new(),
                nick_player: String::new(),
                start_capital: String::new(),
                max_count_player: 6,
                max_start_capital: 1_000_000_000.0,
                enable_bot: false,
            },
            window::change_mode(iced::window::Mode::Fullscreen),
        )
    }

    fn title(&self) -> String {
        String::from("3m")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            //Start-menu block
            Message::StartMenu(StartMenuMessage::StartGameButtonPressed) => {
                self.current_page = Page::Lobby;

                Command::none()
            }
            Message::StartMenu(StartMenuMessage::SettingsButtonPressed) => {
                self.current_page = Page::Settings;

                Command::none()
            }
            Message::StartMenu(StartMenuMessage::ExitButtonPressed) => window::close(),
            //~Start-menu block

            //Lobby block
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

            Message::Lobby(LobbyMessage::NicknamePlayerInputChanged(new_value)) => {
                self.nick_player = new_value;

                Command::none()
            }

            Message::Lobby(LobbyMessage::EnableBotCheckBox(new_value)) => {
                self.enable_bot = new_value;

                Command::none()
            }

            Message::Lobby(LobbyMessage::StartCapitalInputChanged(new_value)) => {
                if new_value == "" {
                    self.start_capital = "".to_string();
                } else {
                    match new_value.parse::<f64>() {
                        Ok(t) => {
                            self.start_capital = if t <= self.max_start_capital {
                                t.to_string()
                            } else {
                                self.start_capital.to_string()
                            }
                        }
                        Err(_) => (),
                    }
                }

                Command::none()
            }

            Message::Lobby(LobbyMessage::StartGame) => {
                if self.is_lobby_value_filled() == true {
                    self.current_page = Page::Game;
                }

                Command::none()
            }
            //~Lobby block

            //Game block
            Message::Game(_) => Command::none(),
            //~Game block
            Message::Settings(_) => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        match self.current_page {
            //View start-menu
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
            //~View start-menu

            //View lobby
            Page::Lobby => {
                //Nickname player block
                let nick_player_row = row![
                    text("Nickname player:").size(20),
                    text_input("", &self.nick_player).on_input(|nick_player| {
                        Message::Lobby(LobbyMessage::NicknamePlayerInputChanged(nick_player))
                    }),
                ]
                .padding(20)
                .spacing(20);
                //~Nickname player block

                //Count player block
                let count_player_row = row![
                    text("Count players:").size(20),
                    text_input("", &self.count_player).on_input(|count_player_value| {
                        Message::Lobby(LobbyMessage::CountPlayerInputChanged(count_player_value))
                    }),
                ]
                .padding(20)
                .spacing(20);
                //~Count player block

                //Start-up capital
                let start_capital_row = row![
                    text("Start-up capital:").size(20),
                    text_input("", &self.start_capital).on_input(|start_capital| {
                        Message::Lobby(LobbyMessage::StartCapitalInputChanged(start_capital))
                    }),
                ]
                .padding(20)
                .spacing(20);
                //~Start-up capital

                //Enable bot for session block
                let bot_enable_row = checkbox("Enable bot", self.enable_bot, |enable_bot| {
                    Message::Lobby(LobbyMessage::EnableBotCheckBox(enable_bot))
                });
                //~Enable bot for session block

                //Result container block
                let content = column![
                    nick_player_row,
                    count_player_row,
                    start_capital_row,
                    bot_enable_row,
                    button("Save settings and start game")
                        .on_press(Message::Lobby(LobbyMessage::StartGame)),
                ]
                .padding(20)
                .spacing(20)
                .align_items(Alignment::Center);
                //~Result container block

                container(content).into()
            }
            //~View lobby

            //View game
            Page::Game => column![button("GameTestButton"),].into(),
            //~View game
            Page::Settings => column![button("SettingsTestButton"),].into(),
        }
    }
}
