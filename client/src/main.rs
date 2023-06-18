use window::Window;
use iced::{Application,Settings};
mod window;

pub fn main() -> iced::Result {
    Window::run(Settings::default())
}
