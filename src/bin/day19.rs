#[macro_use]
extern crate maplit;

use std::io::{self, Read};
use std::str::FromStr;

fn op(code: usize, r: [usize; 6], c: [usize; 4]) -> [usize; 6] {
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
    let ops = hashmap!{
        "addr" => 0,
        "addi" => 1,
        "mulr" => 2,
        "muli" => 3,
        "banr" => 4,
        "bani" => 5,
        "borr" => 6,
        "bori" => 7,
        "setr" => 8,
        "seti" => 9,
        "gtir" => 10,
        "gtri" => 11,
        "gtrr" => 12,
        "eqir" => 13,
        "eqri" => 14,
        "eqrr" => 15,
    };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input);
    let mut ls: Vec<String> = input.lines().map(|x| x.to_owned()).collect();

    let mut commands: Vec<[usize; 4]> = vec![];

    let ip = parse(&ls[0][4..]);

    for xs in &ls[1..] {
        let opcode: String = xs[0..4].to_owned();
        let o = ops[&opcode[..]];
        let abc: Vec<usize> =
            (&xs[5..]).split(" ").map(|x| parse(x)).take(3).collect();
        commands.push([o, abc[0], abc[1], abc[2]]);
    }

    println!("{:?}", commands);

    let mut reg = [0, 10551315, 3, 1, 0, 10551315];
    //let mut reg = [1, 0, 0, 0, 0, 0];
    let mut instr = reg[ip];
    let mut iter = 0;
    while reg[ip] < commands.len() {
        println!("{} -> {:?}", iter, reg);
        reg = op(commands[reg[ip]][0], reg, commands[reg[ip]]);
        reg[ip] += 1;
        iter += 1;
    }

    // 5 is inner loop
    // 3 is outer loop

    // if (3) * (5) == magic {
    //   add (3) to (0)
    // }

    //reset register 3 to 1
    // (2) seti 1 7 5

    //multiply registers 3 and 5 and put in 4
    //(3) mulr 3 5 4

    // check if registers 4 and 1 are equal, set result in 4
    //(4) eqrr 4 1 4

    // add results of registers 2 and 4 and put in 4
    // (increases register 2 by one 'extra' if 4 is equal to 1)
    //(5) addr 4 2 2

    // this effectivly skips (7)
    //(6) addi 2 1 2

    // only reached if 4 and 1 are equal: add register 3 to zero
    //(7) addr 3 0 0

    // increase main counter at 5
    //(8) addi 5 1 5

    // if the main counter is bigger than the fixed number, set 4 to 1
    //(9) gtrr 5 1 4

    // skip 11 if main coutner is bigger than fixed
    //(10) addr 2 4 2

    // set instruction counter to 2 (so back to main loop)
    //(11) seti 2 3 2

    // add 1 to register 3
    //(12) addi 3 1 3

    // check if 3 is greater than 1, store in 4
    //(13) gtrr 3 1 4

    // if (3) is greater than (1) then we skip 15 and go to 16
    // (14) addr 4 2 2

    // reset to (2)
    // (15) seti 1 9 2

    // this is an exit instruction
    // (16) mulr 2 2 2
}
