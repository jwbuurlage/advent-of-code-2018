
extern crate aoc;
use aoc::{List, Pointer};

use std::string::String;
use std::io;

use std::collections::HashMap;

fn main() {
    let mut polymer = String::new();
    io::stdin().read_line(&mut polymer);

    let lower: Vec<char> = String::from("abcdefghijklmnopqrstuvwxyz").chars().collect();
    let upper: Vec<char> = String::from("abcdefghijklmnopqrstuvwxyz").to_uppercase().chars().collect();

    let mut mate = HashMap::<char, char>::new();
    for (i, x) in lower.iter().enumerate() {
        mate.insert(*x, upper[i]);
    }
    for (i, x) in upper.iter().enumerate() {
        mate.insert(*x, lower[i]);
    }


    let mut xs : List<char> = List::new();

    for x in polymer.chars() {
       xs.push_back(x);
    }
    xs.remove(xs.tail);
    println!("{:?}", match xs.tail { Pointer(n) => n });

    let mut n = xs.head;

    let mut prev = '.';
    while !n.is_null() {
        let x = xs[n].value;
        println!("seeing: {}", x);
        if prev == *mate.get(&x).unwrap() {
            println!("match {} {}", prev, x);
            let mut next = xs[xs[n].prev].prev;
            if next.is_null() {
                next = xs[n].next;
            }
            xs.remove(xs[n].prev);
            xs.remove(n);
            n = next;
            prev = '.'
        } else {
            prev = x;
            n = xs[n].next;
        }
    }

    let mut n = xs.head;
    let mut count = 0;
    while !n.is_null() {
        count += 1;
        n = xs[n].next;
    }
    println!("{}", count);
}
