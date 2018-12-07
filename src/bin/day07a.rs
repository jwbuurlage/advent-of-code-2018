extern crate topological_sort;
use topological_sort::TopologicalSort;

extern crate aoc;
use aoc::*;

use std::collections::{BTreeSet, BinaryHeap, HashMap};
use std::error::Error;
use std::fs;
use std::io::{self, Read, Write};
use std::result;
use std::str::FromStr;
use std::string::String;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let xs: Vec<(&str, &str)> = input
        .lines()
        .map(|line| (&line[5..6], &line[36..37]))
        .collect();

    let mut ts = TopologicalSort::<&str>::new();
    for x in &xs {
        println!("{} depends on {}", x.1, x.0);
        ts.add_dependency(x.0, x.1);
    }

    loop {
        let mut x = ts.pop_all();
        if x.len() == 0 {
            break;
        }
        x.sort();
        print!("{}", x[0]);
        if x.len() > 1 {
            for y in &x[1..] {
                for z in &xs {
                    if &z.0 == y {
                        ts.add_dependency(z.0, z.1);
                    }
                }
            }
        }
    }
    println!("");
}
