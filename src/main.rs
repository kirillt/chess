#![feature(box_syntax, box_patterns, duration_as_u128)]
#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate ncurses;
extern crate rand;

mod mode;
use mode::state::{ModeState,
    Empty, SideContainer,
    PreviousLocation};

use mode::property::{Property,
    ColumnOddness, Height, Quadrant,
    KnightDistance};

mod color;
use color::Color;

mod location;

use ncurses::*;
use std::time::{Duration, Instant};
use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let mode = args.next().unwrap();

    initscr();
    printw("Welcome to chess trainer!\n");

    match &mode[..] {
        "oddness" => {
            ColumnOddness::print_help();
            cycle::<Empty, ColumnOddness>(Empty);
        },
        "color" => {
            Color::print_help();
            cycle::<Empty, Color>(Empty);
        },
        "quadrant" => {
            Quadrant::print_help();
            cycle::<SideContainer, Quadrant>(SideContainer::parse(args));
        },
        "height" => {
            Height::print_help();
            cycle::<Empty, Height>(Empty);
        },
        "knight" => {
            KnightDistance::print_help();
            cycle::<PreviousLocation, KnightDistance>(PreviousLocation::parse(args));
        },
        _ => {
            endwin();
            panic!("Unknown mode.")
        }
    };
}

fn cycle<State: ModeState, Prop: Property<State> + PartialEq>(mut state: State) {

    let mut total_time = Duration::new(0, 0);
    let mut total_answers = 0;
    let mut correct_answers = 0;

    loop {
        printw(&format!("{}", state));

        let location = state.next();
        printw(&format!("{}? [", location));

        let answer = Prop::calculate(&state, &location);

        let time = Instant::now();
        let mut guess = None;
        while guess == None {
            guess = Prop::parse(std::char::from_u32(getch() as u32).unwrap());
        }

        let time = time.elapsed();
        total_answers += 1;
        total_time += time;

        let guess = guess.unwrap();

        let result = if guess == answer {
            correct_answers += 1;
            format!("Yes! {}", answer.details())
        } else {
            "No. ".to_string()
        };

        let time  = format!("{:4}", time.as_millis());
        let speed = format!("{:.*}", 2, total_answers as f32 / (total_time.as_secs() as f32));
        let ratio = format!("{:.*}", 2, correct_answers as f32 / total_answers as f32);

        printw(&format!("]: {} Time of thinking: {}ms. Speed: {} answers/sec. Success ratio: {}\n",
            result, time, speed, ratio));

        state.tick(&location);

        refresh();
    }
}
