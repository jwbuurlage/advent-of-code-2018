use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: (i32, i32),
    v: (i32, i32)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let mut grid_a: Vec<Vec<char>> = vec![];
    for (i, line) in input.lines().enumerate() {
        let xs = line.chars().collect();
        grid_a.push(xs);
    }
    let w = grid_a[0].len();
    let h = grid_a.len();
    let mut grid = vec![vec!['#'; w + 2]; h + 2];
    for i in 1..(h + 1) {
        for j in 1..( w + 1 ) {
            grid[i][j] = grid_a[i - 1][j - 1];
        }
    }
    println!("{:?}", grid);



    //let mut grid: Vec<Vec<char>> = vec![];
}
