use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: (i32, i32),
    v: (i32, i32),
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
    let mut grid = vec![vec!['.'; w + 2]; h + 2];
    for i in 1..(h + 1) {
        for j in 1..(w + 1) {
            grid[i][j] = grid_a[i - 1][j - 1];
        }
    }
    println!("{:?}", grid);

    let dx: Vec<i32> = vec![-1, -1, -1, 0, 0, 1, 1, 1];
    let dy: Vec<i32> = vec![-1, 0, 1, -1, 1, -1, 0, 1];

    let mut t = 1;
    loop {
        let mut change = false;
        let mut buf = grid.clone();
        for i in 1..(h + 1) {
            for j in 1..(w + 1) {
                let mut trees = 0;
                let mut yards = 0;
                for k in 0..dx.len() {
                    match grid[((i as i32) + dx[k]) as usize]
                        [((j as i32) + dy[k]) as usize]
                    {
                        '|' => trees += 1,
                        '#' => yards += 1,
                        _ => {}
                    };
                }
                if grid[i][j] == '.' && trees >= 3 {
                    buf[i][j] = '|';
                    change = true;
                }
                if grid[i][j] == '|' && yards >= 3 {
                    buf[i][j] = '#';
                    change = true
                }
                if grid[i][j] == '#' && (trees < 1 || yards < 1) {
                    buf[i][j] = '.';
                    change = true;
                }
            }
        }
        grid = buf;


        let trees = &grid.iter().flatten().filter(|&&x| x == '|').count();
        let yards = &grid.iter().flatten().filter(|&&x| x == '#').count();
        println!("{} -> {}", t, trees * yards);

        t += 1;
        if !change {
            break;
        }
        //        for line in &grid {
        //            let lstr: String = line.iter().collect();
        //            println!("{}", lstr);
        //        }
        //        println!("");
    }


    //let mut grid: Vec<Vec<char>> = vec![];
}
