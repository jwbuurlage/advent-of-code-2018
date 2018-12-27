use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let _result = io::stdin().read_to_string(&mut input);
    let mut stack: Vec<char> = input.chars().collect();
    stack.reverse();

    let counts: Vec<_> = ['N', 'W', 'S', 'E']
        .iter()
        .map(|&x| input.chars().filter(|&c| c == x).count())
        .collect();
    println!("{:?}", counts);

    let mut grid = vec![vec!['#'; 4000]; 4000];

    let mut stack: Vec<((i32, i32), char)> =
        stack.iter().map(|&x| ((-1, -1), x)).collect();

    let mut pos = (2000, 2000);
    grid[pos.0 as usize][pos.1 as usize] = 'X';

    let mut min_i: usize = 2000;
    let mut max_i: usize = 2000;
    let mut min_j: usize = 2000;
    let mut max_j: usize = 2000;

    while let Some(((i, j), x)) = stack.pop() {
        if i >= 0 {
            pos = (i, j);
        }
        match x {
            '^' | '$' | 'P' | '\n' => {
                continue;
            }
            'N' => {
                grid[(pos.0 - 1) as usize][pos.1 as usize] = '-';
                pos.0 -= 2;
                grid[pos.0 as usize][pos.1 as usize] = '.';
            }
            'W' => {
                grid[pos.0 as usize][(pos.1 - 1) as usize] = '|';
                pos.1 -= 2;
                grid[pos.0 as usize][pos.1 as usize] = '.';
            }
            'S' => {
                grid[(pos.0 + 1) as usize][pos.1 as usize] = '-';
                pos.0 += 2;
                grid[pos.0 as usize][pos.1 as usize] = '.';
            }
            'E' => {
                grid[pos.0 as usize][(pos.1 + 1) as usize] = '|';
                pos.1 += 2;
                grid[pos.0 as usize][pos.1 as usize] = '.';
            }
            '(' => {
                // first find closing bracket, and groups between |
                let mut branches: Vec<Vec<char>> = vec![];
                let mut ys = vec![];
                let mut d = 1;
                while let Some(((_, _), x)) = stack.pop() {
                    match x {
                        ')' => {
                            // min counter
                            d -= 1;
                            if d == 0 {
                                branches.push(ys.clone());
                                ys.clear();
                                break;
                            }
                            ys.push(x);
                        }
                        '(' => {
                            d += 1;
                            // add to counter;
                            ys.push(x);
                        }
                        '|' => {
                            // splitting only on pipes that are not within parentheses
                            if d == 1 {
                                branches.push(ys.clone());
                                ys.clear();
                            } else {
                                ys.push(x);
                            }
                        }
                        _ => ys.push(x),
                    }
                }
                // for both branches, copy the tail of the stack, and push both to the stack
                let tail = stack.clone();
                for b in &mut branches {
                    for x in tail.iter() {
                        if x.1 == '$' {
                            break;
                        }
                        stack.push(*x);
                    }
                    for c in b.iter().rev() {
                        stack.push(((-1, -1), *c));
                    }
                    stack.push(((pos.0, pos.1), 'P'));
                }
            }
            _ => {
                println!("Unknown char: {}", x);
                break;
            }
        }

        if (pos.0 as usize) < min_i {
            min_i = pos.0 as usize;
        }
        if (pos.0 as usize) > max_i {
            max_i = pos.0 as usize;
        }
        if (pos.1 as usize) < min_j {
            min_j = pos.1 as usize;
        }
        if (pos.1 as usize) > max_j {
            max_j = pos.1 as usize;
        }
    }

    let mut xs = vec![vec!['#'; max_j - min_j + 3]; max_i - min_i + 3];
    for (i, line) in grid[(min_i - 1)..=(max_i + 1)].iter().enumerate() {
        for (j, c) in line[(min_j - 1)..=(max_j + 1)].iter().enumerate() {
            println!("{}, {}", i, j);
            xs[i][j] = *c;
        }
    }

    for line in &xs {
        let s: String = line.iter().collect();
        println!("{}", s);
    }

    let large = 100000000;
    let mut ds = vec![vec![large; xs[0].len()]; xs.len()];
    let mut s = vec![(0, (2000 - min_i + 1, 2000 - min_j + 1))];
    while let Some((d, (i, j))) = s.pop() {
        if ds[i][j] <= d {
            continue;
        }
        ds[i][j] = d;

        if xs[i][j + 1] == '|' {
            s.push((d + 1, (i, j + 2)));
        }
        if xs[i][j - 1] == '|' {
            s.push((d + 1, (i, j - 2)));
        }
        if xs[i - 1][j] == '-' {
            s.push((d + 1, (i - 2, j)));
        }
        if xs[i + 1][j] == '-' {
            s.push((d + 1, (i + 2, j)));
        }
    }

    let answer = ds.iter().flatten().filter(|&&x| x != large).max().unwrap();
    println!("{}", answer);

    let answer2 = ds
        .iter()
        .flatten()
        .filter(|&&x| x != large)
        .filter(|&&x| x >= 1000)
        .count();
    println!("{}", answer2);
}
