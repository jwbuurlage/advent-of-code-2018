extern crate aoc;
use aoc::*;

use std::collections::BTreeSet;

fn main() {
    let xs = read_all::<i32>();
    let mut f = 0;
    let mut fs = BTreeSet::<i32>::new();

    loop {
        for x in xs.clone() {
            f += x;
            if fs.contains(&f) {
                println!("{}", f);
                return;
            }
            fs.insert(f);
        }
    }
}
