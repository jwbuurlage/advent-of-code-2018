extern crate image;

use std::collections::{BinaryHeap, HashMap, BTreeSet};
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

    let mut grid = vec![vec![(0 as i32, -2 as i32); (h + 1) as usize]; (w + 1) as usize];

    let large = w * h * 2 as i32;
    let mut visited = vec![vec![vec![large + 1; xs.len()]; (h + 1) as usize]; (w + 1) as usize];

    let mut q = BinaryHeap::new();
    for (i, c) in xs.iter().enumerate() {
        q.push(State {
            d: large,
            x: *c,
            i: i as i32,
        });
    }

    println!("{}x{} count {}", w, h, xs.len());

    let print = |g: &Vec<Vec<(i32, i32)>>| {
        for xs in g {
            for y in xs {
                if y.1 == -2 {
                    print!("x ");
                } else if y.1 == -1 {
                    print!(". ");
                } else {
                    print!("{} ", y.1);
                }
            }
            print!("\n");
        }
        print!("\n");
    };

    let mut cur_d = large;
    while let Some(State { d, x: (x, y), i }) = q.pop() {
        if d < cur_d {
            println!("{}, {}", large - d, q.len());
            cur_d = d;
        }
        if grid[x as usize][y as usize].0 == d && grid[x as usize][y as usize].1 != i {
            grid[x as usize][y as usize] = (d, -1); // ambiguous
        } else if grid[x as usize][y as usize].0 > d {
            continue;
        } else {
            grid[x as usize][y as usize] = (d, i);
        }

        for &(a, b) in &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if a >= 0 && a <= w && b >= 0 && b <= h {
                if grid[a as usize][b as usize].0 < d - 1
                    && visited[a as usize][b as usize][i as usize] != d
                {
                    visited[a as usize][b as usize][i as usize] = d;
                    q.push(State {
                        d: d - 1,
                        x: (a, b),
                        i: i,
                    });
                }
            }
        }
    }

    // now we have our grid.. we count
    let mut counts = HashMap::new();
    for i in 0..xs.len() {
        let area_i: usize = grid
            .iter()
            .map(|xs| xs.iter().filter(|x| x.1 == i as i32).count())
            .sum();
        counts.insert(i, area_i);
    }

    println!("{:?}", counts);

    let mut infinites = BTreeSet::new();

    for x in 0..w+1 {
        infinites.insert(grid[x as usize][0 as usize].1);
        infinites.insert(grid[x as usize][h as usize].1);
    }
    for y in 0..h+1 {
        infinites.insert(grid[0 as usize][y as usize].1);
        infinites.insert(grid[w as usize][y as usize].1);
    }

    println!("{:?}", infinites);
    for x in infinites {
        let y = x as usize;
        if (counts.contains_key(&y)) {
            counts.remove(&y);
        }
    }

    println!("{:?}", counts);
    println!("a: {}", counts.iter().max_by_key(|x| x.1).unwrap().1);

    // debug output
    let grid_as_u8: Vec<u8> = grid.into_iter().flatten().map(|x| unsafe { (x.1 * 5) as u8 }).collect();
    image::save_buffer(
        "grid.png",
        &grid_as_u8[..],
        (h + 1) as u32,
        (w + 1) as u32,
        image::Gray(8),
    );
}
