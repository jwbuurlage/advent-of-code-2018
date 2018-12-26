use std::collections::{BinaryHeap, HashSet};
use std::io::{self, Read};

fn op(code: usize, r: [usize; 4], c: [usize; 4]) -> [usize; 4] {
    let mut result = r;
    match (code) {
        0 => {
            result[c[3]] = r[c[1]] + r[c[2]];
        } // addr
        1 => {
            result[c[3]] = r[c[1]] + c[2];
        } // addi
        2 => {
            result[c[3]] = r[c[1]] * r[c[2]];
        } // multr
        3 => {
            result[c[3]] = r[c[1]] * c[2];
        } // addltii
        4 => {
            result[c[3]] = r[c[1]] & r[c[2]];
        } // banr
        5 => {
            result[c[3]] = r[c[1]] & c[2];
        } // bani
        6 => {
            result[c[3]] = r[c[1]] | r[c[2]];
        } // borr
        7 => {
            result[c[3]] = r[c[1]] | c[2];
        } // bori
        8 => {
            result[c[3]] = r[c[1]];
        } // setr
        9 => {
            result[c[3]] = c[1];
        } // seti
        10 => {
            result[c[3]] = (c[1] > r[c[2]]) as usize;
        } // gtir
        11 => {
            result[c[3]] = (r[c[1]] > c[2]) as usize;
        } // gtri
        12 => {
            result[c[3]] = (r[c[1]] > r[c[2]]) as usize;
        } // gtrr
        13 => {
            result[c[3]] = (c[1] == r[c[2]]) as usize;
        } // eqir
        14 => {
            result[c[3]] = (r[c[1]] == c[2]) as usize;
        } // eqri
        15 => {
            result[c[3]] = (r[c[1]] == r[c[2]]) as usize;
        } // eqrr
        _ => {
            assert!(false);
        }
    }
    return result;
}

fn parse(x: &str) -> usize {
    return x.trim().parse::<usize>().unwrap();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let mut ls = input.lines();
    let mut cs = vec![vec![true; 16]; 16];

    let mut total = 0;
    loop {
        let a = ls.next().unwrap();
        let xs = ls.next().unwrap();
        if a == "" {
            break;
        }

        let b = ls.next().unwrap();
        let e = ls.next().unwrap();

        let mut acc = 0;

        let r = [
            parse(&a[9..10]),
            parse(&a[12..13]),
            parse(&a[15..16]),
            parse(&a[18..19]),
        ];
        let mut c = [0, 0, 0, 0];
        let cvec: Vec<usize> =
            xs.split(" ").map(|x| parse(x)).take(4).collect();
        c[0] = cvec[0];
        c[1] = cvec[1];
        c[2] = cvec[2];
        c[3] = cvec[3];

        let o = [
            parse(&b[9..10]),
            parse(&b[12..13]),
            parse(&b[15..16]),
            parse(&b[18..19]),
        ];

        for i in 0..16 {
            let output = op(i, r, c);
            if output == o {
                acc += 1;
            } else {
                // remove c[0] from i
                cs[i][c[0]] = false;
            }
        }

        if (acc >= 3) {
            total += 1;
        }
    }

    println!("total: {}", total);

    let mut convert = vec![0; 16];
    loop {
        let mut change = false;
        for i in 0..16 {
            if cs[i].iter().filter(|&x| *x).count() == 1 {
                for j in 0..16 {
                    if cs[i][j] {
                        change = true;
                        convert[j] = i;
                        for k in 0..16 {
                            cs[k][j] = false;
                        }
                    }
                }
            }
        }
        if !change { break; }
    }


    let mut reg = [0, 0, 0, 0];
    while let Some(xs) = ls.next() {
        let mut c = [0, 0, 0, 0];
        let cvec: Vec<usize> =
            xs.split(" ").map(|x| parse(x)).take(4).collect();
        c[0] = cvec[0];
        c[1] = cvec[1];
        c[2] = cvec[2];
        c[3] = cvec[3];

        reg = op(convert[c[0]], reg, c);
    }

    println!("registers: {:?}", reg);
}
