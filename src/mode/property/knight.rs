use mode::state::PreviousLocation;
use mode::property::Property;

use location::Location;

use ncurses::*;

use std::cmp;
use std::u8;

#[derive(PartialEq)]
pub struct KnightDistance {
    value: u8
}

impl KnightDistance {
    pub fn calculate(from: &Location, to: &Location) -> u8 {
        PATHS[from.column as usize]
             [from.row    as usize]
             [to.column as usize]
             [to.row    as usize]
    }
}

impl Property<PreviousLocation> for KnightDistance {
    fn parse(guess: char) -> Option<Box<KnightDistance>> {
        format!("{}", guess).parse::<u8>().ok()
            .map(|x| box KnightDistance { value: x })
    }

    fn calculate(previous: &PreviousLocation, location: &Location) -> Box<KnightDistance> {
        box KnightDistance {
            value: KnightDistance::calculate(&previous.location, location)
        }
    }

    fn print_help() {
        printw("=========================================================\n");
        printw("              KNIGHT DISTANCE GUESSING MODE\n");
        printw("---------------------------------------------------------\n");
        printw("Type [1], [2], [3] or [4] if you think this number equals\n");
        printw("  to amount of knight's moves from 1st to 2nd position.\n");
        printw("=========================================================\n");
    }
}

const MOVES_AMOUNT: usize = 8;

lazy_static! {
    static ref PATHS: [[[[u8; 8]; 8]; 8]; 8] = calculate_all();

    static ref MOVES: [(i8,i8); MOVES_AMOUNT] = [
        (-1,2),(-1,-2),(-2,1),(-2,-1),
        (2,1),(2,-1),(1,2),(1,-2)
    ];
}

fn calculate_all() -> [[[[u8; 8]; 8]; 8]; 8] {
    let mut paths = [[[[u8::MAX; 8]; 8]; 8]; 8];

    for x0 in 0..8 {
        for y0 in 0..8 {
            for k in 0..MOVES_AMOUNT {
                let (x1, y1) = match MOVES[k] {
                    (dx, dy) => (x0 + dx as usize, y0 + dy as usize)
                };
                if in_bounds(x1) && in_bounds(y1) {
                    paths[x0][y0][x1][y1] = 1;
                }
            }
        }
    }

    for k in 0..64 {
        for i in 0..64 {
            for j in 0..64 {
                let current = distance(paths, i, j);
                let candidate = distance(paths, i, k) + distance(paths, k, j);

                update(&mut paths, i, j, cmp::min(current, candidate));
            }
        }
    }

    paths
}

fn update(paths: &mut [[[[u8; 8]; 8]; 8]; 8], i: usize, j: usize, distance: u8) {
    let (ix,iy) = split(i);
    let (jx,jy) = split(j);
    paths[ix][iy][jx][jy] = distance;
}

fn distance(paths: [[[[u8; 8]; 8]; 8]; 8], i: usize, j: usize) -> u8 {
    let (ix,iy) = split(i);
    let (jx,jy) = split(j);
    paths[ix][iy][jx][jy]
}

fn split(combined: usize) -> (usize, usize) {
    (combined / 8, combined % 8)
}

fn in_bounds(x: usize) -> bool {
    x < 8
}