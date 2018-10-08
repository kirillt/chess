use mode::*;

use ncurses::*;

#[derive(PartialEq)]
pub enum ColumnOddness { Odd, Even }

impl Property for ColumnOddness {
    fn parse(guess: char) -> Option<Box<ColumnOddness>> {
        match guess {
            'o' => Some(box ColumnOddness::Odd),
            '1' => Some(box ColumnOddness::Odd),
            'e' => Some(box ColumnOddness::Even),
            '2' => Some(box ColumnOddness::Even),

            _ => None
        }
    }

    fn calculate(_side: &Option<Color>, column: u8, _row: u8) -> Box<ColumnOddness> {
        match (column + 1) % 2 {
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
