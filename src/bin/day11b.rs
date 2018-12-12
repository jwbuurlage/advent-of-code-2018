use std::io::{self, Read};
use std::str::FromStr;

fn main() {
    let serial = 9306;

    let mut grid = vec![vec![0; 302]; 302];

    for x in 1..301 {
        for y in 1..301 {
            let mut power_level: i32 = ((x + 10) * y + serial) * (x + 10);
            power_level = ((power_level / 100) % 10) - 5;
            grid[x as usize][y as usize] = power_level;
        }
    }

    let mut score = grid.clone();
    for x in 1..301 {
        for y in 1..301 {
            score[x][y] = grid[x][y];
        }
    }

    println!("pre");
    for xs in &score[0..5] {
        println!("{:?}", &xs[0..5]);
    }
    println!("/pre");

    let mut total = vec![vec![vec![0; 302]; 302]; 302];

    for y in 1..301 {
        for x in 1..301 {
            score[x][y] += score[x - 1][y];
        }
    }
    for y in 1..301 {
        for x in 1..301 {
            score[x][y] += score[x][y - 1];
        }
    }

    println!("post");
    for xs in &score[0..5] {
        println!("{:?}", &xs[0..5]);
    }
    println!("/post");


    for x in 1..301 {
        for y in 1..301 {
            for z in 0..300 {
                if (x + z > 300) || (y + z > 300) {
                    continue;
                }
                total[x][y][z] = score[x + z][y + z] + score[x - 1][y - 1]
                    - score[x + z][y - 1]
                    - score[x - 1][y + z];
            }
        }
    }

    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_z = 0;
    let mut max = -100000;
    for x in 1..301 {
        for y in 1..301 {
            for z in 0..300 {
                if (x + z > 300) || (y + z > 300) {
                    break;
                }
                if total[x][y][z] > max {
                    println!("{} > {} @ {}, {}, {}", total[x][y][z], max, x, y, z);
                    best_x = x;
                    best_y = y;
                    best_z = z;
                    max = total[x][y][z];
                }
            }
        }
    }

    println!("total");
    for x in 88..92 {
        for y in 268..270 {
            print!("{} ", total[x][y][15]);
        }
        println!("");
    }
    println!("/total");

    println!(
        "{},{},{} (-> {})",
        best_x, best_y, best_z + 1, total[best_x as usize][best_y as usize][best_z as usize]
    );
}
