pub trait Property {
    fn parse(guess: char) -> Option<Box<Self>>;
    fn calculate(column: u8, row: u8) -> Box<Self>;
    fn print_help();
}

pub mod color;
pub mod quadrant;
pub mod height;