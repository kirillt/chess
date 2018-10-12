use mode::state::Empty;
use mode::property::Property;

use location::Location;

use ncurses::*;

#[derive(PartialEq)]
pub enum ColumnOddness { Odd, Even }

impl Property<Empty> for ColumnOddness {
    fn parse(guess: char) -> Option<Box<ColumnOddness>> {
        match guess {
            'o' => Some(box ColumnOddness::Odd),
            '1' => Some(box ColumnOddness::Odd),
            'e' => Some(box ColumnOddness::Even),
            '2' => Some(box ColumnOddness::Even),

            _ => None
        }
    }

    fn calculate(_state: &Empty, location: &Location) -> Box<ColumnOddness> {
        match (location.column + 1) % 2 {
            0 => box ColumnOddness::Even,
            1 => box ColumnOddness::Odd,
            _ => panic!()
        }
    }

    fn print_help() {
        printw("===================================================================\n");
        printw("                   COLUMN ODDNESS GUESSING MODE\n");
        printw("-------------------------------------------------------------------\n");
        printw("Type [e] or [2] if you think column of specified position is [even]\n");
        printw("and type [o] or [1] if you think the column is [odd].\n");
        printw("===================================================================\n");
    }
}
