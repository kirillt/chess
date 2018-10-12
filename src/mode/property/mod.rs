use mode::state::ModeState;

pub trait Property<State: ModeState> {
    fn parse(guess: char) -> Option<Box<Self>>;

    fn calculate(state: &State, column: u8, row: u8) -> Box<Self>;

    fn print_help();
}

pub mod color;
pub mod quadrant;
pub mod height;
pub mod column;

pub use self::column::ColumnOddness;
pub use self::quadrant::Quadrant;
pub use self::height::Height;