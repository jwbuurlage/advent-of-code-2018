use std::io::{self, Read};
use std::str::FromStr;

fn main() {
    // TODO add `gen` falses to the left
    let gen = 200;

    let strinput = "#...#####.#..##...##...#.##.#.##.###..##.##.#.#..#...###..####.#.....#..##..#.##......#####..####...";
    let mut strinput_bool: Vec<bool> = strinput
        .chars()
        .map(|x| match x {
            '#' => true,
            _ => false,
        })
        .collect();
    let mut input: Vec<bool> = vec![false; gen];
    input.append(&mut strinput_bool);
    input.append(&mut vec![false; gen]);

    let mut filein = String::new();
    io::stdin().read_to_string(&mut filein);

    let mut rule = vec![false; 32];

    for line in filein.lines() {
        let mut rule_base: Vec<bool> = line[0..5]
            .chars()
            .map(|x| match x {
                '#' => true,
                _ => false,
            })
            .collect();

        let magic = rule_base
            .iter()
            .enumerate()
            .fold(0, |sum, (i, x)| if *x { sum + (1 << i) } else { sum });

        println!("{:?}, {}", rule_base, magic);

        rule[magic] = match &line[9..10] {
            "#" => true,
            _ => false,
        };
    }

    for x in 2..(input.len() - 2) {
        print!(
            "{}",
            match input[x] {
                true => '#',
                false => '.',
            }
        );
    }
    println!("");


    let mut result = 0;
    for i in 0..gen {
        let mut input_new = input.clone();
        for x in 2..(input.len() - 2) {
            let magic = input[x - 2..=x + 2]
                .iter()
                .enumerate()
                .fold(0, |sum, (i, x)| if *x { sum + (1 << i) } else { sum });
            input_new[x] = rule[magic];
        }
        input = input_new;
        for x in 2..(input.len() - 2) {
            print!(
                "{}",
                match input[x] {
                    true => '#',
                    false => '.',
                }
            );
        }
        println!("");

        let old_result = result;
        result = 0;
        for i in 0..input.len() {
            if input[i] {
                result += (i as i32) - (gen as i32);
            }
        }

        println!("{:?} - {} = {}", result, old_result, result - old_result);
    }

    let mut result = 0;
    for i in 0..input.len() {
        if input[i] {
            result += (i as i32) - (gen as i32);
        }
    }

    println!("{} {:?}", gen, result);
}
