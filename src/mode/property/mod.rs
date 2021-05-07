use crate::location::Location;
use crate::mode::state::ModeState;

pub trait Property<State: ModeState> {
    fn parse(guess: char) -> Option<Box<Self>>;

    fn calculate(state: &State, location: &Location) -> Box<Self>;

    fn details(&self) -> String {
        "".to_string()
    }

    fn print_help();
}

pub mod color;
pub mod column;
pub mod height;
pub mod knight;
pub mod quadrant;

pub use self::column::ColumnOddness;
pub use self::height::Height;
pub use self::knight::KnightDistance;
pub use self::quadrant::Quadrant;
