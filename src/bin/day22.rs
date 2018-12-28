use std::collections::BinaryHeap;

fn main() {
    let depth = 3339;
    let w = 2000;
    let h = 2000;
    let target = (10, 715);
    let magic = 20183;

    let mut grid = vec![vec![0; w]; h];
    for i in 0..w {
        grid[0][i] = (i * 16807 + depth) % magic;
    }

    for j in 0..h {
        grid[j][0] = (j * 48271 + depth) % magic;
    }

    for j in 1..h {
        for i in 1..w {
            grid[j][i] = (grid[j][i - 1] * grid[j - 1][i] + depth) % magic;
        }
    }

    grid[0][0] = 0;
    grid[target.1][target.0] = 0;

    // set w, h to target + 1 for this to be correct
    //    let ans: usize = grid.iter().flatten().map(|x| x % 3).sum();
    //    println!("ans: {}", ans);

    for j in 0..h {
        for i in 0..w {
            grid[j][i] = grid[j][i] % 3;
        }
    }

    // do a DIJKSTRA
    let mut visited = vec![vec![vec![false; w]; h]; 3];
    let large = 100000000000;
    let mut q = BinaryHeap::new();
    q.push((large, (0, 0), 0));

    let dx: Vec<i32> = vec![-1, 0, 1, 0];
    let dy: Vec<i32> = vec![0, 1, 0, -1];

    // 0 rocky
    // 1 wet
    // 2 narrow

    // 0 torch
    // 1 climbing
    // 2 none

    let xs = [[0, 1], [1, 2], [0, 2]];

    while let Some((d, (x, y), t)) = q.pop() {
        if visited[t][y][x] {
            continue;
        }
        if x == target.0 && y == target.1 {
            println!("{}", large - d);
            break;
        }
        visited[t][y][x] = true;

        for k in 0..4 {
            if (x as i32) + dx[k] >= 0
                && (y as i32) + dy[k] >= 0
                && (((x as i32) + dx[k]) as usize) < w
                && (((y as i32) + dy[k]) as usize) < h
            {
                let mut d2 = d - 1;
                let mut t2 = t;

                let a = ((x as i32) + dx[k]) as usize;
                let b = ((y as i32) + dy[k]) as usize;

                if !xs[grid[b][a]].contains(&t) {
                    if grid[y][x] == 0 {
                        t2 = (t + 1) % 2;
                    } else if grid[y][x] == 1 {
                        t2 = ((t - 1) % 2) + 1;
                    } else if grid[y][x] == 2 {
                        if t == 0 {
                            t2 = 2;
                        } else {
                            t2 = 0;
                        }
                    }
                    d2 -= 7;
                }

                if a == target.0 && b == target.1 {
                    if t2 != 0 {
                        d2 -= 7;
                    }
                }

                q.push((d2, (a, b), t2))
            }
        }
    }

    //for line in grid {
    //    let s: String = line
    //        .iter()
    //        .map(|&x| {
    //            let t = x % 3;
    //            match t {
    //                0 => '.',
    //                1 => '=',
    //                2 => '|',
    //                _ => '?',
    //            }
    //        })
    //        .collect();
    //    println!("{}", s);
    //}
}
