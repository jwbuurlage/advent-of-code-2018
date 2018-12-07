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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State<'a> {
    t: i32,
    s: &'a str,
}

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


    let ord = |s: &str| -> i32 { (s.as_bytes()[0] as i32) -  ("A".as_bytes()[0] as i32)};
    let time = |s: &str| 61 + ord(s);
    let mut visited = vec![false; 26];

    let mut workers_available = 5;
    let mut q = BinaryHeap::new();

    let mut ys: Vec<&str> = ts.peek_all().iter().map(|x| -> &str { *x }).collect();
    ys.sort();
    for x in ys.iter().take(workers_available) {
        q.push(State { t: -time(x), s: x });
        visited[ord(x) as usize] = true;
        workers_available -= 1;
    }
    while let Some(State { t, s }) = q.pop() {
        println!("{} {}", t, s);

        let ys = ts.pop_all();
        for y in ys {
            if y != s {
                for z in &xs {
                    if &z.0 == &y {
                        ts.add_dependency(z.0, z.1);
                    }
                }
            }
        }

        workers_available += 1;

        let mut ys: Vec<&str> =
            ts.peek_all().iter().map(|x| -> &str { *x }).collect();
        ys.sort();
        for y in ys.iter() {
            if visited[ord(y) as usize] { continue; }
            visited[ord(y) as usize] = true;
            q.push(State {
                t: t - time(y),
                s: y,
            });
            workers_available -= 1;
            if workers_available == 0 {
                break;
            }
        }
    }
}
