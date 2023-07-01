use iced::widget::canvas::{Canvas, Cursor, Frame, Geometry, Path,Program};
use iced::{Color, Element, Rectangle, Size, Theme};

#[derive(Debug, Clone)]
pub enum Message {}

// First, we define the data we need for drawing
#[derive(Debug)]
pub struct Square {
    width: f32,
    height: f32,
}

// Then, we implement the `Program` trait
impl Program<Message> for Square {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        // We prepare a new `Frame`
        let mut frame = Frame::new(bounds.size());

        // We create a `Path` representing a simple circle
        let square = Path::rectangle(frame.center(), Size::new(self.width, self.height));

        // And fill it with some color
        frame.fill(&square, Color::BLACK);

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }
}

impl Square {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(self.width)
            .height(self.height)
            .into()
    }
}
