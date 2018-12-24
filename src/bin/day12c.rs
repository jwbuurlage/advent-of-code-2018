#[macro_use]
extern crate maplit;

use std::io::{self, Read};
use std::str::FromStr;

fn main() {
    let gen = 20;
    let mut state: Vec<char> = "#...#####.#..##...##...#.##.#.##.###..##.##.#.#..#...###..####.#.....#..##..#.##......#####..####...".to_owned().chars().collect();
    let mut buffer = vec!['.'; gen];
    buffer.extend(state);
    buffer.extend(vec!['.'; gen]);

    let mut state = buffer;

    let rules = hashmap!{
        "#.#.#" => '#',
        "..###" => '.',
        "#..#." => '#',
        ".#..." => '#',
        "..##." => '#',
        "##.#." => '#',
        "##..#" => '#',
        "####." => '#',
        "...#." => '#',
        "..#.#" => '#',
        ".####" => '#',
        "#.###" => '.',
        "...##" => '.',
        "..#.." => '.',
        "#...#" => '.',
        ".###." => '#',
        ".#.##" => '.',
        ".##.." => '#',
        "....#" => '.',
        "#..##" => '.',
        "##.##" => '#',
        "#.##." => '.',
        "#...." => '.',
        "##..." => '#',
        ".#.#." => '.',
        "###.#" => '#',
        "#####" => '#',
        "#.#.." => '.',
        "....." => '.',
        ".##.#" => '.',
        "###.." => '.',
        ".#..#" => '.',
    };

    let step = |state: &mut Vec<char>| {
        let state_copy = state.clone();
        for i in 2..(state.len()-2) {
            let llnrr: String = state_copy[i-2..i+3].iter().collect();
            state[i] = rules[&llnrr[..]]
        }
    };

    for i in 0..gen {
        step(&mut state);
        let state_str: String = state.iter().collect();
        println!("{:?}", state_str);
    }

    let state_str: String = state.iter().collect();
    println!("{:?}", state_str);

    let mut result =0;
    for i in 0..state.len() {
        if state[i] == '#' {
            result += i - gen;
        }
    }

    println!("{}", result)
}
