use mode::*;

use ncurses::*;

#[derive(PartialEq)]
pub enum Quadrant { TopRight, BottomRight, BottomLeft, TopLeft }

impl Property for Quadrant {
    fn parse(guess: char) -> Option<Box<Quadrant>> {
        match guess {
            '1' => Some(box Quadrant::BottomLeft),
            '3' => Some(box Quadrant::BottomRight),
            '7' => Some(box Quadrant::TopLeft),
            '9' => Some(box Quadrant::TopRight),

            _ => None
        }
    }

    fn calculate(column: u8, row: u8) -> Box<Quadrant> {
        box if row < 4 {
            if column < 4 { Quadrant::BottomLeft } else { Quadrant::BottomRight }
        } else {
            if column < 4 { Quadrant::TopLeft } else { Quadrant::TopRight }
        }
    }

    fn print_help() {
        printw("==========================================================\n");
        printw("                   QUADRANT GUESSING MODE\n");
        printw("----------------------------------------------------------\n");
        printw("              Use NUM-section on your keyboard:\n");
        printw("              ---------------------------------\n");
        printw("                  [1] means [left-bottom]\n");
        printw("                  [3] means [right-bottom]\n");
        printw("                  [7] means [left-top]\n");
        printw("                  [9] means [right-top]\n");
        printw("==========================================================\n");
    }
}