use lazy_static::lazy_static;

use crate::mode::property::Property;
use crate::mode::state::PreviousLocation;

use crate::location::Location;

use ncurses::*;

use std::u8;

pub struct KnightDistance {
    penultimate: Option<Location>,
    value: u8,
}

impl PartialEq for KnightDistance {
    fn eq(&self, other: &KnightDistance) -> bool {
        self.value == other.value
    }
}

impl KnightDistance {
    pub fn distance(from: &Location, to: &Location) -> u8 {
        DISTANCES[from.file as usize][from.rank as usize][to.file as usize][to.rank as usize]
    }

    pub fn penultimate(from: &Location, to: &Location) -> (usize, usize) {
        PREDECESSORS[from.file as usize][from.rank as usize][to.file as usize][to.rank as usize]
    }
}

impl Property<PreviousLocation> for KnightDistance {
    fn parse(guess: char) -> Option<Box<KnightDistance>> {
        format!("{}", guess).parse::<u8>().ok().map(|x| {
            Box::new(KnightDistance {
                penultimate: None,
                value: x,
            })
        })
    }

    fn calculate(previous: &PreviousLocation, location: &Location) -> Box<KnightDistance> {
        let (i, j) = KnightDistance::penultimate(&previous.location, location);
        Box::new(KnightDistance {
            penultimate: Some(Location {
                file: i as u8,
                rank: j as u8,
            }),
            value: KnightDistance::distance(&previous.location, location),
        })
    }

    fn details(&self) -> String {
        format!("(via {})", self.penultimate.as_ref().unwrap())
    }

    fn print_help() {
        addstr("=========================================================\n");
        addstr("              KNIGHT DISTANCE GUESSING MODE\n");
        addstr("---------------------------------------------------------\n");
        addstr("Type [1], [2], [3] or [4] if you think this number equals\n");
        addstr("  to amount of knight's moves from 1st to 2nd position.\n");
        addstr("=========================================================\n");
    }
}

const MOVES_AMOUNT: usize = 8;

type Matrix4D = [[[[u8; 8]; 8]; 8]; 8];

type BiMatrix4D = [[[[(usize, usize); 8]; 8]; 8]; 8];

lazy_static! {
    static ref DISTANCES: Matrix4D = PATHS.0;
    static ref PREDECESSORS: BiMatrix4D = PATHS.1;
    static ref PATHS: (Matrix4D, BiMatrix4D) = calculate_all();
    static ref MOVES: [(i8, i8); MOVES_AMOUNT] = [
        (-1, 2),
        (-1, -2),
        (-2, 1),
        (-2, -1),
        (2, 1),
        (2, -1),
        (1, 2),
        (1, -2)
    ];
}

fn calculate_all() -> (Matrix4D, BiMatrix4D) {
    let mut distances: Matrix4D = [[[[255; 8]; 8]; 8]; 8];
    let mut predecessors: BiMatrix4D = [[[[(255, 255); 8]; 8]; 8]; 8];

    for x0 in 0..8 {
        for y0 in 0..8 {
            for k in 0..MOVES_AMOUNT {
                let (x1, y1) = match MOVES[k] {
                    (dx, dy) => (x0 + dx as usize, y0 + dy as usize),
                };
                if in_bounds(x1) && in_bounds(y1) {
                    predecessors[x0][y0][x1][y1] = (x0, y0);
                    distances[x0][y0][x1][y1] = 1;
                }
            }
        }
    }

    for k in 0..64 {
        for i in 0..64 {
            for j in 0..64 {
                let current = distance(distances, i, j);
                let candidate = distance(distances, i, k) + distance(distances, k, j);

                if current > candidate {
                    let (ix, iy) = split(i);
                    let (jx, jy) = split(j);

                    predecessors[ix][iy][jx][jy] = predecessor(predecessors, k, j);
                    distances[ix][iy][jx][jy] = candidate;
                }
            }
        }
    }

    (distances, predecessors)
}

fn predecessor(predecessors: BiMatrix4D, i: usize, j: usize) -> (usize, usize) {
    let (ix, iy) = split(i);
    let (jx, jy) = split(j);
    predecessors[ix][iy][jx][jy]
}

fn distance(distances: Matrix4D, i: usize, j: usize) -> u8 {
    let (ix, iy) = split(i);
    let (jx, jy) = split(j);
    distances[ix][iy][jx][jy]
}

fn split(combined: usize) -> (usize, usize) {
    (combined / 8, combined % 8)
}

fn in_bounds(x: usize) -> bool {
    x < 8
}
