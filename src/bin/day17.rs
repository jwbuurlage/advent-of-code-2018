#[macro_use]
extern crate nom;

use std::io::{self, Read, Write};
use std::str::FromStr;

#[derive(Debug)]
enum Vein {
    Hor((usize, usize), usize),
    Ver(usize, (usize, usize)),
}

named!(int<&str, usize>,
       map!(nom::digit, |x| -> usize { FromStr::from_str(x).expect("error") })
);

named!(hor<&str, Vein>,
    do_parse!(
       tag!("y=")
       >> y: int
       >> tag!(", x=")
       >> xmin: int
       >> tag!("..")
       >> xmax: int
       >> tag!("\n")
       >> (Vein::Hor ( (xmin, xmax), y ))
    )
);

named!(ver<&str, Vein>,
       do_parse!(
           tag!("x=")
               >> x: int
               >> tag!(", y=")
               >> ymin: int
               >> tag!("..")
               >> ymax: int
               >> tag!("\n")
               >> (Vein::Ver ( x, (ymin, ymax) ))
       )
);

named!(vein<&str, Vein>,
       alt!(hor | ver)
);

named!(veins<&str, Vec<Vein>>,
       many1!(vein)
);

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let mut veins: Vec<Vein> =
        veins(&input[..]).expect("could not parse input").1;

    let mut bbox = [[10000, 10000], [0, 0]];
    for v in &veins {
        match *v {
            Vein::Hor((xmin, xmax), y) => {
                if bbox[0][0] > xmin {
                    bbox[0][0] = xmin;
                }
                if bbox[1][0] < xmax {
                    bbox[1][0] = xmax;
                }
                if bbox[0][1] > y {
                    bbox[0][1] = y;
                }
                if bbox[1][1] < y {
                    bbox[1][1] = y;
                }
            }
            Vein::Ver(x, (ymin, ymax)) => {
                if bbox[0][1] > ymin {
                    bbox[0][1] = ymin;
                }
                if bbox[1][1] < ymax {
                    bbox[1][1] = ymax;
                }
                if bbox[0][0] > x {
                    bbox[0][0] = x;
                }
                if bbox[1][0] < x {
                    bbox[1][0] = x;
                }
            }
        };
    }
    let min_y = bbox[0][1];
    bbox[0][1] = 0;
    bbox[0][0] -= 1;
    bbox[1][0] += 1;
    println!("{:?}", bbox);

    for v in &mut veins {
        match *v {
            Vein::Hor((xmin, xmax), y) => {
                *v = Vein::Hor(
                    (xmin - bbox[0][0], xmax - bbox[0][0]),
                    y - bbox[0][1],
                );
            }
            Vein::Ver(x, (ymin, ymax)) => {
                *v = Vein::Ver(
                    x - bbox[0][0],
                    (ymin - bbox[0][1], ymax - bbox[0][1]),
                );
            }
        };
    }

    let w = bbox[1][0] - bbox[0][0] + 1;
    let h = bbox[1][1] - bbox[0][1] + 1;

    let mut grid = vec![vec!['.'; w + 1]; h + 1];

    for v in &veins {
        match (*v) {
            Vein::Hor((xmin, xmax), i) => {
                for j in xmin..=xmax {
                    grid[i][j] = '#';
                }
            }
            Vein::Ver(j, (ymin, ymax)) => {
                for i in ymin..=ymax {
                    grid[i][j] = '#';
                }
            }
        }
    }

    grid[0][500 - bbox[0][0]] = '+';

    let mut stack = vec![(0, 500 - bbox[0][0])];

    while let Some((i, j)) = stack.pop() {
        if i + 1 > bbox[1][1] {
            continue;
        }

        if grid[i + 1][j] == '.' {
            stack.push((i + 1, j));
            grid[i + 1][j] = '|';
        } else if grid[i + 1][j] != '|' {
            let mut settle = true;

            let mut a = j;
            while grid[i][a] != '#' {
                a -= 1;

                if !(grid[i + 1][a] == '#' || grid[i + 1][a] == '~') {
                    settle = false;
                    a -= 1;
                    break;
                }
            }

            let mut b = j;
            while (grid[i][b] != '#') {
                b += 1;
                if !(grid[i + 1][b] == '#' || grid[i + 1][b] == '~') {
                    settle = false;
                    b += 1;
                    break;
                }
            }

            if settle {
                for k in (a + 1)..b {
                    grid[i][k] = '~';
                }
                stack.push((i - 1, j));
            } else {
                for k in (a + 1)..b {
                    grid[i][k] = '|';
                }
                if b - 1 != j && a + 1 != j {
                    stack.push((i, b - 1));
                    stack.push((i, a + 1));
                }
            }
        }

        //        for line in &grid {
        //            let lstr: String = line.iter().collect();
        //            println!("{}", lstr);
        //        }
        //        println!("");
    }

    let wet = &grid[min_y..]
        .iter()
        .flatten()
        .filter(|&&x| x == '|' || x == '~')
        .count();

    for line in &grid {
        let lstr: String = line.iter().collect();
        println!("{}", lstr);
    }
    println!("");

    println!("{}", wet);

    let dried = &grid[min_y..]
        .iter()
        .flatten()
        .filter(|&&x| x == '~')
        .count();
    println!("{}", dried);
}
