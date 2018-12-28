#[macro_use]
extern crate nom;

// TODO
//do_parse!(...);

struct Army {
    t: i32, // team
    u: i32, // units
    w: Vec<String>, // weaknesses
    m: Vec<String>, // immunities
    d: i32, // damage
    a: String, // attack type
    i: i32, // initiative
}

fn main() {
    let ans = 0;
    println!("ans: {}", ans);
}
