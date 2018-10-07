#[derive(PartialEq)]
pub enum Color { White, Black }

impl Color {
    pub fn invert(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }

    pub fn parse_str(input: String) -> Option<Color> {
        match &input[..] {
            "white" => Some(Color::White),
            "black" => Some(Color::Black),
            "w" => Some(Color::White),
            "b" => Some(Color::Black),

            _ => None
        }
    }

    pub fn print(&self) -> &str {
        match &self {
            Color::White => "white",
            Color::Black => "black"
        }
    }
}

pub trait Property {
    fn parse(guess: char) -> Option<Box<Self>>;

    fn calculate(side: &Color, column: u8, row: u8) -> Box<Self>;

    fn print_help();
}

pub mod color;
pub mod quadrant;
pub mod height;
pub mod column;
