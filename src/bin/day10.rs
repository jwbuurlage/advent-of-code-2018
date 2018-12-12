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

    let mut points : Vec<Point> = input.lines().map(|line| {
        let convert = |s : &str| FromStr::from_str(s.to_string().trim()).unwrap();
        Point{x: (convert(&line[10..16]),convert(&line[18..24])), v: (convert(&line[36..38]),convert(&line[40..42]))}
    }).collect();

    println!("{:?}", &points);

    let size = 200;
    let mut grid = vec![vec!['.'; size]; size];
    let mut stats = |i, points: &Vec<Point>| {
        let ax = points.iter().map(|p| p.x.0).min().unwrap();
        let ay = points.iter().map(|p| p.x.1).min().unwrap();
        let bx = points.iter().map(|p| p.x.0).max().unwrap();
        let by = points.iter().map(|p| p.x.1).max().unwrap();
        println!("{}: w = [{}, {}] x [{}, {}]", i, ax, bx, ay, by);
        if (bx - ax < size as i32) && (by - ay < size as i32) {
            grid = vec![vec!['.'; size]; size];
            for p in points {
                grid[(p.x.0 - ax) as usize][(p.x.1 - ay) as usize] = '#';
            }
            for i in 0..size {
                for j in 0..size {
                    print!("{}", grid[i][j]);
                }
                println!("");
            }
            println!("");
        }
    };

    for i in 0..10510 {
        for p in &mut points {
            p.x.0 += p.v.0;
            p.x.1 += p.v.1;
        }
    }

    for i in 0..5 {
        stats(i, &points);
        for p in &mut points {
            p.x.0 += p.v.0;
            p.x.1 += p.v.1;
        }
    }
}
