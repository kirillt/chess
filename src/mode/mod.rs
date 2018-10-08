use color::Color;

pub trait Property {
    fn parse(guess: char) -> Option<Box<Self>>;

    fn calculate(side: &Option<Color>, column: u8, row: u8) -> Box<Self>;

    fn print_help();
}

pub mod color;
pub mod quadrant;
pub mod height;
pub mod column;
