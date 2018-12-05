extern crate aoc;
use aoc::{List, Pointer, deep_copy};

use std::io;
use std::string::String;

use std::collections::HashMap;

fn main() {
    let mut polymer = String::new();
    io::stdin().read_line(&mut polymer);

    let lower: Vec<char> = String::from("abcdefghijklmnopqrstuvwxyz").chars().collect();
    let upper: Vec<char> = String::from("abcdefghijklmnopqrstuvwxyz")
        .to_uppercase()
        .chars()
        .collect();

    let mut mate = HashMap::<char, char>::new();
    for (i, x) in lower.iter().enumerate() {
        mate.insert(*x, upper[i]);
    }
    for (i, x) in upper.iter().enumerate() {
        mate.insert(*x, lower[i]);
    }

    let mut xs: List<char> = List::new();

    for x in polymer.chars() {
        xs.push_back(x);
    }
    xs.remove(xs.tail);
    println!(
        "{:?}",
        match xs.tail {
            Pointer(n) => n,
        }
    );

    for x in lower {
        let mut ys = deep_copy(&xs);
        // remove x and X
        let mut n = ys.head;
        while !n.is_null() {
            let next = ys[n].next;
            let value = ys[n].value;
            if value == x || value == *mate.get(&x).unwrap() {
                ys.remove(n);
            }
            n = next;
        }

        let mut n = ys.head;
        let mut prev = '.';
        while !n.is_null() {
            let x = ys[n].value;
            if prev == *mate.get(&x).unwrap() {
                let mut next = ys[ys[n].prev].prev;
                if next.is_null() {
                    next = ys[n].next;
                }
                ys.remove(ys[n].prev);
                ys.remove(n);
                n = next;
                prev = '.'
            } else {
                prev = x;
                n = ys[n].next;
            }
        }

        let mut n = ys.head;
        let mut count = 0;
        let mut string = String::new();
        while !n.is_null() {
            count += 1;
            string.push(ys[n].value);
            n = ys[n].next;
        }
        println!("{}{} {}", x, *mate.get(&x).unwrap(), count);
    }
}
