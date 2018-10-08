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

    fn calculate(side: &Option<Color>, column: u8, row: u8) -> Box<Quadrant> {
        let side = side.as_ref().unwrap();

        let quadrant = if row < 4 {
            if column < 4 { Quadrant::BottomLeft } else { Quadrant::BottomRight }
        } else {
            if column < 4 { Quadrant::TopLeft } else { Quadrant::TopRight }
        };

        box if side == &Color::Black {
            Quadrant::invert(quadrant)
        } else {
            quadrant
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

impl Quadrant {
    fn invert(self) -> Quadrant {
        match self {
            Quadrant::TopRight    => Quadrant::BottomLeft,
            Quadrant::TopLeft     => Quadrant::BottomRight,
            Quadrant::BottomRight => Quadrant::TopLeft,
            Quadrant::BottomLeft  => Quadrant::TopRight,
        }
    }
}
