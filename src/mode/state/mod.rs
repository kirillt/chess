use std::fmt::{Display, Formatter, Result};

use location::Location;

pub trait ModeState: Display {
    fn next(&self) -> Location {
        Location::random()
    }

    fn tick(&mut self, location: &Location);
}

pub struct Empty;

impl Display for Empty {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "")
    }
}

impl ModeState for Empty {
    fn tick(&mut self, _location: &Location) { }
}

pub mod side;
pub mod previous;

pub use self::side::SideContainer;
pub use self::previous::PreviousLocation;