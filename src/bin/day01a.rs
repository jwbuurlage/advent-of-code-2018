extern crate aoc;
use aoc::*;

fn main() {
    let xs = read_all::<i32>();
    let result : i32 = xs.iter().sum();
    println!("{}", result)
}
