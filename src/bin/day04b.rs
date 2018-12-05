#[macro_use]
extern crate nom;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::str::FromStr;

// ------------------------------------------------------------------------------
// Data
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Date {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{}-{} {}:{}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Event {
    Start(Date, usize),
    Wake(Date),
    Sleep(Date),
}

impl Event {
    fn date(self) -> Date {
        return match self {
            Event::Start(d, _) => d,
            Event::Wake(d) => d,
            Event::Sleep(d) => d,
        };
    }

    // time in minutes between two events
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        self.date().cmp(&other.date())
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        self.date().partial_cmp(&other.date())
    }
}

// ------------------------------------------------------------------------------
// Parsers
named!(int<&str, usize>,
    map!(nom::digit, |x| -> usize { FromStr::from_str(x).expect("error") })
);

named!(date<&str, Date>,
       do_parse!(
           tag!("[")
               >> year: int
               >> tag!("-")
               >> month: int
               >> tag!("-")
               >> day: int
               >> tag!(" ")
               >> hour: int
               >> tag!(":")
               >> minute: int
               >> tag!("]")
               >> (Date { year, month, day, hour, minute })
       )
);

named!(start<&str, Event>,
       do_parse!(
       d: date
       >> tag!(" Guard #")
       >> id: int
       >> tag!(" begins shift\n")
       >> (Event::Start(d, id))
       )
);

named!(wake<&str, Event>,
       do_parse!(
           d: date
            >> tag!(" wakes up\n")
            >> (Event::Wake(d))
       )
);

named!(sleep<&str, Event>,
       do_parse!(
           d: date
               >> tag!(" falls asleep\n")
               >> (Event::Sleep(d))
       )
);

named!(event<&str, Event>,
   alt!(start | wake | sleep)
);

named!(events<&str, Vec<Event>>,
       many1!(event)
);

// ------------------------------------------------------------------------------
// Algorithm
fn main() {
    let file = fs::read("data/input04.txt").expect("failed to open file");
    let str_file = String::from_utf8(file).expect("could not parse as utf8");
    let mut all = events(&str_file[..]).expect("could not parse events").1;

    all.sort();
    //println!("{:?}", all);

    // lets check which dates are in
    // let shifts: Vec<Date> = all
    //     .iter()
    //     .filter(|event| match (event) {
    //         Event::Start(_, _) => true,
    //         _ => false,
    //     })
    //     .map(|x| x.date())
    //     .collect();
    // for d in shifts {
    //     println!("{}", d)
    // }

    // simulate
    // we assume the data is more or less 'sane':
    // - one guard per night
    // - data is of the form: start, (sleep, awake)*

    let mut schedule = HashMap::<usize, [usize; 60]>::new(); // total times asleep during a certain minute

    let mut active_guard: usize = 0;
    let mut last_minute: usize = 0;
    let mut asleep: bool = false;
    for e in all {
        match e {
            Event::Start(d, i) => {
                println!("{} {} start", d, i);
                // finish simulating previous shift
                if asleep {
                    for i in last_minute..60 {
                        if let Some(xs) = schedule.get_mut(&active_guard) {
                            xs[i] += 1;
                        }
                    }
                }
                // start next shift
                active_guard = i;
                if !schedule.contains_key(&active_guard) {
                    schedule.insert(active_guard, [0; 60]);
                }
                if d.hour == 23 {
                    last_minute = 0;
                } else {
                    last_minute = d.minute;
                }
            }
            Event::Sleep(d) => {
                println!("{} sleep", d);
                last_minute = d.minute;
                asleep = true;
            }
            Event::Wake(d) => {
                println!("{} wake", d);
                assert!(asleep, "wake while guard was not asleep");
                for i in last_minute..d.minute {
                    if let Some(xs) = schedule.get_mut(&active_guard) {
                        xs[i] += 1;
                    }
                }
                asleep = false;
            }
        }
    }

    let mut totals: Vec<(usize, (usize, &usize))> = schedule
        .iter()
        .map(|guard| -> (usize, (usize, &usize)) {
            (
                *guard.0,
                guard.1.iter().enumerate().max_by_key(|&(_, b)| b).unwrap(),
            )
        })
        .collect();
    totals.sort_by(|(_, (_, x)), (_, (_, y))| x.cmp(y));
    println!("{:?}", totals);

    println!("{:?}", totals[totals.len() - 1].0 * (totals[totals.len() - 1].1).0);
}
