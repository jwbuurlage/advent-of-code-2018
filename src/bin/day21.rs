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

    let mut reg = [0, 0, 0, 0, 0, 0];
    let mut instr = reg[ip];
    let mut iter = 0;
    while reg[ip] < commands.len() {
        reg = op(commands[reg[ip]][0], reg, commands[reg[ip]]);

        if (reg[ip] == 26) {
            println!("{} -> {:?}", iter, reg);
        }

        reg[ip] += 1;
        iter += 1;
    }


    // loop 1
    // this loop increases (1) by one, until ((1) + 1) * 256 > (2)
    // when this happens, we go to instruction 26
    // (1) starts at 0 here

    // (1) = 0
    // 17 seti 0 8 1

    // (5) = (1) + 1
    //18 addi 1 1 5

    // (5) = 256 * (5)
    //19 muli 5 256 5

    // (5) = (5) > (2)
    //20 gtrr 5 2 5

    // (4) = (4) + (5)
    //21 addr 5 4 4

    // (4) = (4) + 1
    //22 addi 4 1 4

    // (4) = 25
    //23 seti 25 5 4

    // (1) = (1) + 1
    //addi 1 1 1

    // (4) = 17
    //25 seti 17 1 4

    // state after this loop:
    // (1a) = (2) / 256
    // (4a) = 26
    // (5a) = 1


    // loop 2
    // this changes (3) based on the value in (1)
    // and eventually exits if this value is equal to the one in register (0)
    // (1a) = 256 > (1)
    // (2) = (1)
    // (3) = ((((3) + (255 & (1))) & 16777215) * 65899) & 16777215

    // (2) = (1)
    //26. setr 1 2 2

    // (4) = 7
    //27. seti 7 0 4

    // (1) = 255 & (2)
    //8   bani 2 255 1

    // (3) = (3) + (1)
    // 9   addr 3 1 3

    // (3) = (3) & 16777215
    //10  bani 3 16777215 3

    // (3) = (3) * 65899
    //11  muli 3 65899 3

    // (3) = (3) & 16777215
    //12  bani 3 16777215 3

    // (1) = 256 > (2)
    //13  gtir 256 2 1

    // (4) = (1) + (4)
    // interpret as: if 256 > (2)
    //14  addr 1 4 4

    // (4) = (4) + 1
    //15  addi 4 1 4

    // (4) = 27
    //16  seti 27 6 4

    // (1) = (3) == (0)
    //28 eqrr 3 0 1

    // (4) = (1) + (4)
    //29 addr 1 4 4

    // (4) = 5
    //30 seti 5 0 4
    // .. goes to sequence 3

    // sequence 3
    // (2) = (3) | 65536
    // 6 bori 3 65536 2
    // (3) = 1099159
    // 7 seti 1099159 8 3

    // ... continues to loop 1
    // (2a) = (3) | 65536
    // (3a) = 1099159



}
