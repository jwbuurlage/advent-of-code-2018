#[macro_use]
extern crate nom;

use std::collections::HashSet;
use std::fs;
use std::io::{self, Read, Write};
use std::str::FromStr;

use nom::types::CompleteStr as Input;

#[derive(Clone, Debug)]
struct Army {
    t: usize,       // team
    u: usize,       // units
    h: usize,       // hit points
    w: Vec<String>, // weaknesses
    m: Vec<String>, // immunities
    d: usize,       // damage
    a: String,      // attack type
    i: usize,       // initiative
    x: bool,        // died
}

named!(int<Input, usize>,
       map!(nom::digit, |x| -> usize { FromStr::from_str(&x).expect("error") })
);

named!(string<Input, String>,
    map!(many1!(one_of!("abcdefghijklmnopqrstuvwxyz")), |x| x.into_iter().collect())
);

named!(comma_sep<Input, Vec<String>>,
       separated_list!(tag!(", "), string)
);

named!(weaknesses<Input, Vec<String>>,
       do_parse!(
           tag!("weak to ")
               >> x: comma_sep
               >> (x)
       )
);

named!(immunities<Input, Vec<String>>,
           do_parse!(
            tag!("immune to ")
            >> x: comma_sep
            >> (x)
            )
);

named!(stats<Input, (Vec<String>, Vec<String>)>,
        alt!(
     delimited!(tag!(" ("),
               // FIXME
                alt!(
                    do_parse!(
                        m: immunities
                        >> tag!("; ")
                        >> w: weaknesses
                        >> ((m, w))
                    ) |
                    do_parse!(
                        w: weaknesses
                        >> tag!("; ")
                        >> m: immunities
                        >> ((m, w))
                    ) |
                    do_parse!(
                        w: weaknesses
                        >> ((vec![], w))
                    ) |
                    do_parse!(
                        m: immunities
                        >> ((m, vec![]))
                    )
                    ),
            tag!(")")
    ) | value!((vec![], vec![])))
);

named_args!(army(t: usize)<Input, Army>,
       do_parse!(
           u: int
               >> tag!(" units each with ")
               >> h: int
               >> tag!(" hit points")
               >> mw: stats
               >> tag!(" with an attack that does ")
               >> d: int
               >> tag!(" ")
               >> a: string
               >> tag!(" damage at initiative ")
               >> i: int
               >> opt!(tag!("\n"))
               >> (Army{t, u, h, w: mw.1, m: mw.0, d, a: a, i, x: false})
       )
);

named!(armies<Input, (Vec<Army>, Vec<Army>)>,
do_parse!(
    tag!("Immune System:\n")
    >> xs: many1!(call!(army, 0))
    >> tag!("\nInfection:\n")
    >> ys: many1!(call!(army, 1))
    >> opt!(tag!("\n"))
    >> (xs, ys)
)
);

fn damage(attacking: &Army, defending: &Army) -> usize {
    if defending.m.contains(&attacking.a) {
        return 0;
    }
    let base = attacking.d * attacking.u;
    if defending.w.contains(&attacking.a) {
        return base * 2;
    }
    return base;
}

fn parse(x: &str) -> usize {
    return x.trim().parse::<usize>().unwrap();
}

fn main() {
    let file = fs::read("data/input24.txt").expect("failed to open file");
    let str_file = String::from_utf8(file).expect("could not parse as utf8");
    let mut armies = armies(Input(&str_file[..])).expect("could not parse").1;
    armies.0.append(&mut armies.1);
    let armies_in = armies.0;

    for iter in 1.. {
        let mut armies = armies_in.clone();
        for army in &mut armies {
            if army.t == 0 {
                army.d += iter;
            }
        }

        loop {
            // Phase 1: targets
            let mut order = (0..armies.len()).collect::<Vec<usize>>();
            order.sort_by(|&i, &j| {
                let xi = (armies[i].d * armies[i].u, armies[i].i);
                let xj = (armies[j].d * armies[j].u, armies[j].i);
                return xj.cmp(&xi);
            });

            let large = order.len() + 1;
            let mut available = vec![true; order.len()];
            let mut targets = vec![large; order.len()];

            for &i in &order {
                let target: usize = (0..armies.len())
                    .filter(|&j| armies[j].t != armies[i].t && available[j])
                    .max_by(|&j, &k| {
                        let xj = (
                            damage(&armies[i], &armies[j]),
                            armies[j].d * armies[j].u,
                            armies[j].i,
                        );
                        let xk = (
                            damage(&armies[i], &armies[k]),
                            armies[k].d * armies[k].u,
                            armies[k].i,
                        );
                        xj.cmp(&xk)
                    })
                    .unwrap_or(large);

                //println!("{} ({:?}) targets {} ({})", i, (armies[i].d * armies[i].u, armies[i].i), target, large);

                if target < order.len() {
                    if damage(&armies[i], &armies[target]) > 0 {
                        targets[i] = target;
                        available[target] = false;
                    }
                }
            }

            // Phase 2: attacks
            order.sort_by(|&i, &j| {
                return armies[j].i.cmp(&armies[i].i);
            });

            let mut change = false;
            for &i in &order {
                if armies[i].x {
                    continue;
                }
                if targets[i] >= order.len() {
                    continue;
                }

                // i attacks target[i];
                let d = damage(&armies[i], &armies[targets[i]]);
                let ul = d / armies[targets[i]].h;
                if ul > 0 {
                    if ul >= armies[targets[i]].u {
                        armies[targets[i]].x = true;
                        change = true;
                    } else {
                        armies[targets[i]].u -= ul;
                        change = true;
                    }
                }
            }

            // remove dead armies
            armies.retain(|a| !a.x);

            if !change {
                break;
            }

            // Phase 3: exit check
            let mut teams: HashSet<usize> = HashSet::new();
            for army in &armies {
                teams.insert(army.t);
            }
            if teams.len() == 1 {
                break;
            }
        }

        // Phase 3: exit check
        let mut teams: HashSet<usize> = HashSet::new();
        for army in &armies {
            teams.insert(army.t);
        }
        if teams.len() != 1 {
            println!("{}. no change", iter);
            continue;
        }

        // get answer
        let ans: usize = armies.iter().map(|a| a.u).sum();
        println!("{}. ans: {} ({})", iter, armies[0].t, ans);

        if armies[0].t == 0 {
            break;
        }
    }
}
