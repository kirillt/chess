#[derive(Clone, Debug, PartialEq)]
pub enum Color { White, Black }

impl Color {
    pub fn invert(&self) -> Self {
        match self {
            &Color::White => Color::Black,
            &Color::Black => Color::White
        }
    }

    pub fn parse_str(input: &str) -> Option<Color> {
        match input {
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


