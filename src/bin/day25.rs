use std::io::{self, Read};
use std::str::FromStr;
use std::collections::BTreeSet;

fn parse(x: &str) -> i32 {
    return x.trim().parse::<i32>().unwrap();
}

fn d(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut d = 0;
    for i in 0..a.len() {
        d += (a[i] - b[i]).abs();
    }
    return d;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let xs: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split(",").map(|x| parse(x)).collect())
        .collect();

    let mut ls: Vec<usize> = (0..xs.len()).collect();

    loop {
        let mut was_mut = false;
        for i in 0..xs.len() {
            for j in i..xs.len() {
                if d(xs[i].clone(), xs[j].clone()) <= 3 {
                    if ls[i] != ls[j] {
                        was_mut = true;
                        if ls[i] < ls[j] {
                            ls[j] = ls[i];
                        } else {
                            ls[i] = ls[j];
                        }
                    }
                }
            }
        }
        if !was_mut {
            break;
        }
    }

    let mut fs = BTreeSet::new();
    for l in ls {
        fs.insert(l);
    }

    println!("{}", fs.len());
}
