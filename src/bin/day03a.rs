#[macro_use]
extern crate nom;

extern crate aoc;
use aoc::*;

use std::collections::HashMap;
use std::fs;
use std::iter::*;
use std::str::FromStr;

#[derive(Debug)]
struct Rect {
    id: usize,
    min: (usize, usize),
    max: (usize, usize),
}

named!(int<&str, usize>,
    map!(nom::digit, |x| -> usize { FromStr::from_str(x).expect("error") })
);

named!(rect<&str, Rect>,
    do_parse!(
        tag!("#")
        >> id: int
        >> tag!(" @ ")
        >> x: int
        >> tag!(",")
        >> y: int
        >> tag!(": ")
        >> w: int
        >> tag!("x")
        >> h: int
        >> tag!("\n")
        >> (Rect { id: id - 1, min: (x, y), max: (x + w, y + h) })
    )
);

named!(rects<&str, Vec<Rect>>,
       many1!(rect)
);

fn main() {
    let file = fs::read("data/input03.txt").expect("failed to open file");
    let str_file = String::from_utf8(file).expect("could not parse as utf8");
    let mut patches = rects(&str_file[..]).expect("could not parse rects").1;

    let mut grid = vec![vec![0; 1000]; 1000];
    for patch in patches {
        for x in patch.min.0..patch.max.0 {
            for y in patch.min.1..patch.max.1 {
                grid[x][y] += 1;
            }
        }
    }

    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            count += ((grid[x][y] >= 2) as usize);
        }
    }

    println!("{}", count)


