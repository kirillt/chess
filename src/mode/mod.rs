use std::fmt::{Display, Formatter, Result};

pub trait ModeState: Display {
    fn tick(&mut self);
}


pub struct Empty;

impl Display for Empty {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "")
    }
}

impl ModeState for Empty {
    fn tick(&mut self) { }
}


pub trait Property<State: ModeState> {
    fn parse(guess: char) -> Option<Box<Self>>;

    fn calculate(state: &State, column: u8, row: u8) -> Box<Self>;

    fn print_help();
}

pub mod color;
pub mod quadrant;
pub mod height;
pub mod column;

pub mod side;