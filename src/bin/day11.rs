use std::io::{self, Read};
use std::str::FromStr;

fn main() {
    let serial = 9306;

    let mut grid = vec![vec![0; 302]; 302];

    for x in 1..301 {
        for y in 1..301 {
            let mut power_level : i32 = ((x + 10) * y + serial) * (x + 10);
            power_level = ((power_level / 100) % 10) - 5;
            grid[x as usize][y as usize] = power_level;
        }
    }

    let mut score = vec![vec![0; 302]; 302];

    for x in 1..301 {
        for y in 1..301 {
            score[x][y] += grid[x - 1][y - 1];
            score[x][y] += grid[x - 1][y];
            score[x][y] += grid[x - 1][y + 1];
            score[x][y] += grid[x][y - 1];
            score[x][y] += grid[x][y];
            score[x][y] += grid[x][y + 1];
            score[x][y] += grid[x + 1][y - 1];
            score[x][y] += grid[x + 1][y];
            score[x][y] += grid[x + 1][y + 1];
        }
    }

    let mut best_x = 0;
    let mut best_y = 0;
    let mut max = -100000;
    for x in 1..301 {
        for y in 1..301 {
            if score[x][y] > max {
                best_x = x;
                best_y = y;
                max = score[x][y];
            }
        }
    }

    println!("{}, {} -> {}", best_x - 1, best_y - 1, max);
}
