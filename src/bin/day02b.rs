extern crate aoc;
use aoc::*;

use std::collections::HashMap;
use std::iter::*;

fn main() {
    let xss = read_all::<String>();
    let yss = xss.clone();

    for xs in xss {
        for ys in yss.clone() {
            let d = xs
                .chars()
                .zip(ys.chars())
                .fold(0, |d, (x, y)| d + ((x != y) as usize));
            if d == 1 {
                println!(
                    "{}",
                    xs.chars()
                        .zip(ys.chars())
                        .filter(|(x, y)| x == y)
                        .map(|(x, _)| x)
                        .collect::<String>()
                );
                return;
            }
        }
    }
}
