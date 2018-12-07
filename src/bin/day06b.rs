use std::collections::{BTreeSet, BinaryHeap, HashMap};
use std::io;
use std::string::String;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    d: i32,
    x: (i32, i32),
    i: i32,
}

fn main() {
    let mut coordinate = String::new();
    let mut xs = Vec::<(i32, i32)>::new();
    while let Ok(x) = io::stdin().read_line(&mut coordinate) {
        if x <= 0 {
            break;
        }
        let xy: Vec<i32> = coordinate
            .split(',')
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();
        xs.push((xy[0], xy[1]));
        coordinate = "".to_string();
    }

    let ax = xs.iter().map(|c| c.0).min().unwrap();
    let ay = xs.iter().map(|c| c.1).min().unwrap();
    let bx = xs.iter().map(|c| c.0).max().unwrap();
    let by = xs.iter().map(|c| c.1).max().unwrap();

    xs = xs.iter().map(|c| (c.0 - ax, c.1 - ay)).collect();
    let w = bx - ax; // width minus one; max coord
    let h = by - ay; // height minus one; max coord

    // I think that if a region reaches the border, it is for sure infinite, so
    // we never have to go outside it

    let mut grid = vec![vec![0; (h + 1) as usize]; (w + 1) as usize];

    let large = w * h * 2 as i32;
    let mut visited = vec![
        vec![vec![false; xs.len()]; (h + 1) as usize];
        (w + 1) as usize
    ];

    let mut q = BinaryHeap::new();
    for (i, c) in xs.iter().enumerate() {
        q.push(State {
            d: large,
            x: *c,
            i: i as i32,
        });
    }

    let mut cur_d = large;
    while let Some(State { d, x: (x, y), i }) = q.pop() {
        grid[x as usize][y as usize] += large - d;

        for &(a, b) in &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if a >= 0 && a <= w && b >= 0 && b <= h {
                if !visited[a as usize][b as usize][i as usize] {
                    visited[a as usize][b as usize][i as usize] = true;
                    q.push(State {
                        d: d - 1,
                        x: (a, b),
                        i: i,
                    });
                }
            }
        }
    }

    let area = grid
        .into_iter()
        .flatten()
        .filter(|&x| x < 10000)
        .count();
    println!("b: {}", area);
}
