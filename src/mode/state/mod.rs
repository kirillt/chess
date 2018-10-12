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

pub mod side;
pub mod previous;

pub use self::side::SideContainer;