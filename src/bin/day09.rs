extern crate aoc;
use aoc::{List, Pointer};

#[macro_use]
extern crate nom;

use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Input {
    players: usize,
    marbles: usize,
}

named!(int<&str, usize>,
       map!(terminated!(nom::digit, tag!(" ")), |x| -> usize { FromStr::from_str(x).expect("error") })
);

named!(node<&str, Input>,
       do_parse!(
           players: int
               >> tag!("players; last marble is worth ")
               >> marbles: int
               >> tag!("points")
               >> (Input { players, marbles })));

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let game = node(&input[..]).expect("could not parse input").1;

    let mut xs: List<usize> = List::new();
    xs.push_back(0);

    let mut scores = vec![0; game.players];
    let mut player = 0;
    let mut current = xs.head;
    let mut marble = 1;
    let mut removed = 0;

    loop {
        if marble % 23 != 0 {
            let mut next = xs[current].next;
            if next.is_null() {
                next = xs.head;
            }
            current = xs.insert_after(next, marble)
        } else {
            scores[player] += marble;
            let mut to_remove = current;
            for i in 0..7 {
                to_remove = xs[to_remove].prev;
                if to_remove.is_null() {
                    to_remove = xs.tail;
                }
            }
            current = xs[to_remove].next;
            if current.is_null() {
                current = xs.head;
            }
            scores[player] += xs.remove(to_remove);
        }
        marble += 1;
        player = (player + 1) % game.players;
        if marble > game.marbles {
            break;
        }
    }

    println!("{}", scores.iter().max().unwrap())
}
