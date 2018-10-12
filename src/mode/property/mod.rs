use location::Location;
use mode::state::ModeState;

pub trait Property<State: ModeState> {
    fn parse(guess: char) -> Option<Box<Self>>;

    fn calculate(state: &State, location: &Location) -> Box<Self>;

    fn print_help();
}

pub mod color;
pub mod quadrant;
pub mod height;
pub mod column;
pub mod knight;

pub use self::knight::KnightDistance;
pub use self::column::ColumnOddness;
pub use self::quadrant::Quadrant;
pub use self::height::Height;