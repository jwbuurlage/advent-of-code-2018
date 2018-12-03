extern crate aoc;
use aoc::*;

use std::collections::HashMap;

fn main() {
    let mut a = 0;
    let mut b = 0;

    let xss = read_all::<String>();
    for xs in xss {
        let mut fs = HashMap::<char, usize>::new();
        for x in xs.chars() {
            if let Some(count) = fs.get_mut(&x) {
                *count += 1;
            } else {
                fs.insert(x, 1);
            }
        }

        if fs.values().any(|&x| x == 2) { a += 1}
        if fs.values().any(|&x| x == 3) { b += 1}
    }
    println!("{}", a * b)
}
