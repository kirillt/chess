use mode::property::Property;
use mode::state::side::SideContainer;

use color::Color;
use location::Location;

use ncurses::*;

#[derive(PartialEq)]
pub enum Quadrant {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

impl Property<SideContainer> for Quadrant {
    fn parse(guess: char) -> Option<Box<Quadrant>> {
        match guess {
            '1' => Some(Box::new(Quadrant::BottomLeft)),
            '3' => Some(Box::new(Quadrant::BottomRight)),
            '7' => Some(Box::new(Quadrant::TopLeft)),
            '9' => Some(Box::new(Quadrant::TopRight)),

            _ => None,
        }
    }

    fn calculate(state: &SideContainer, location: &Location) -> Box<Quadrant> {
        let quadrant = if location.rank < 4 {
            if location.file < 4 {
                Quadrant::BottomLeft
            } else {
                Quadrant::BottomRight
            }
        } else {
            if location.file < 4 {
                Quadrant::TopLeft
            } else {
                Quadrant::TopRight
            }
        };

        Box::new(if state.side == Color::Black {
            Quadrant::invert(quadrant)
        } else {
            quadrant
        })
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
            Quadrant::TopRight => Quadrant::BottomLeft,
            Quadrant::TopLeft => Quadrant::BottomRight,
            Quadrant::BottomRight => Quadrant::TopLeft,
            Quadrant::BottomLeft => Quadrant::TopRight,
        }
    }
}
