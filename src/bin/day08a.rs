#[macro_use]
extern crate nom;

use std::io::{self, Read, Write};
use std::str::FromStr;
use std::string::String;

#[derive(Debug)]
struct Node {
    meta: Vec<usize>,
    childs: Vec<Node>,
}

named!(int<&str, usize>,
       map!(terminated!(nom::digit, tag!(" ")), |x| -> usize { FromStr::from_str(x).expect("error") })
);

named!(node<&str, Node>,
       do_parse!(
       qn: int
       >> qm: int
       >> childs: count!(node, qn)
       >> meta: count!(int, qm)
       >> (Node { meta, childs })));

fn combine(x: &Node) -> usize {
    let own: usize = x.meta.iter().sum();
    let kids: usize = x.childs.iter().map(|x| combine(x)).sum();
    return own + kids;
}

fn value(x: &Node) -> usize {
    if x.childs.len() == 0 {
        return x.meta.iter().sum();
    } else {
        let mut sum = 0;
        for &y in &x.meta {
            if y >= 1 && y <= x.childs.len() {
                sum += value(&x.childs[y - 1])
            }
        }
        return sum;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let node = node(&input[..]).expect("could not parse nodes").1;
    println!("nodes: {:?}", node);

    println!("ans: {:?}", combine(&node));
    println!("ans2: {:?}", value(&node));
}
