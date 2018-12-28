use std::cmp;
use std::collections::BinaryHeap;
use std::io::{self, Read};
use std::str::FromStr;

fn parse(x: &str) -> i32 {
    return x.trim().parse::<i32>().unwrap();
}

fn mdist(
    x: (i32, i32, i32),
    cube: ((i32, i32), (i32, i32), (i32, i32)),
) -> i32 {
    let mut d = 0;

    if (cube.0).0 > x.0 || (cube.0).1 < x.0 {
        d += std::cmp::min(((cube.0).0 - x.0).abs(), ((cube.0).1 - x.0).abs());
    }

    if (cube.1).0 > x.1 || (cube.1).1 < x.1 {
        d += std::cmp::min(((cube.1).0 - x.1).abs(), ((cube.1).1 - x.1).abs());
    }

    if (cube.2).0 > x.2 || (cube.2).1 < x.2 {
        d += std::cmp::min(((cube.2).0 - x.2).abs(), ((cube.2).1 - x.2).abs());
    }

    return d;
}

fn empty(cube: ((i32, i32), (i32, i32), (i32, i32))) -> bool {
    return ((cube.0).0 > (cube.0).1)
        || ((cube.1).0 > (cube.1).1)
        || ((cube.2).0 > (cube.2).1);
}

fn single(cube: ((i32, i32), (i32, i32), (i32, i32))) -> bool {
    return ((cube.0).1 == ((cube.0).0))
        && ((cube.1).1 == ((cube.1).0))
        && ((cube.2).1 == ((cube.2).0));
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let xs: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split(" ").map(|x| parse(x)).collect())
        .collect();

    println!("{:?}", xs);

    let m = xs
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x[3].cmp(&y[3]))
        .unwrap();

    let mut ans = 0;
    for (i, bot) in xs.iter().enumerate() {
        let d = (bot[0] - m.1[0]).abs()
            + (bot[1] - m.1[1]).abs()
            + (bot[2] - m.1[2]).abs();
        if d <= m.1[3] {
            ans += 1;
        }
    }
    println!("{}", ans);

    let x1 = xs.iter().map(|x| x[0]).min().unwrap();
    let x2 = xs.iter().map(|x| x[0]).max().unwrap();
    let y1 = xs.iter().map(|x| x[1]).min().unwrap();
    let y2 = xs.iter().map(|x| x[1]).max().unwrap();
    let z1 = xs.iter().map(|x| x[2]).min().unwrap();
    let z2 = xs.iter().map(|x| x[2]).max().unwrap();

    let mut q = BinaryHeap::new();
    q.push((xs.len(), 0, ((x1, x2), (y1, y2), (z1, z2))));

    while let Some((c, d, ((x1, x2), (y1, y2), (z1, z2)))) = q.pop() {
        println!(
            "{} {} (({}, {}),({}, {}), ({}, {}))",
            c, d, x1, x2, y1, y2, z1, z2
        );
        let xm = (x1 + x2) / 2;
        let ym = (y1 + y2) / 2;
        let zm = (z1 + z2) / 2;

        if single(((x1, x2), (y1, y2), (z1, z2))) {
            println!("ans: {} ({})", d, c);
            break;
        }

        let splits = vec![
            ((x1, xm), (y1, ym), (zm + 1, z2)),
            ((x1, xm), (y1, ym), (z1, zm)),
            ((x1, xm), (ym + 1, y2), (zm + 1, z2)),
            ((x1, xm), (ym + 1, y2), (z1, zm)),
            ((xm + 1, x2), (y1, ym), (zm + 1, z2)),
            ((xm + 1, x2), (y1, ym), (z1, zm)),
            ((xm + 1, x2), (ym + 1, y2), (zm + 1, z2)),
            ((xm + 1, x2), (ym + 1, y2), (z1, zm)),
        ];

        for s in splits {
            if empty(s) || s == ((x1, x2), (y1, y2), (z1, z2)) {
                continue;
            }

            let mut c = 0;
            for b in 0..xs.len() {
                if mdist((xs[b][0], xs[b][1], xs[b][2]), s) <= xs[b][3] {
                    c += 1;
                }
            }

            q.push((c, mdist((0, 0, 0), s), s))
        }
    }
}
