#[macro_use]
extern crate nom;

extern crate aoc;
use aoc::*;

struct Date {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
}

struct Guard {
    id: i32,
    awakes: Vec<(Date, Date)>,
}

named!(int<&str, usize>,
    map!(nom::digit, |x| -> usize { FromStr::from_str(x).expect("error") })
);

named!(rect<&str, Rect>,
    do_parse!(
        tag!("#")
        >> id: int
        >> tag!(" @ ")
        >> x: int
        >> tag!(",")
        >> y: int
        >> tag!(": ")
        >> w: int
        >> tag!("x")
        >> h: int
        >> tag!("\n")
        >> (Rect { id: id - 1, min: (x, y), max: (x + w, y + h) })
    )
);

named!(rects<&str, Vec<Rect>>,
       many1!(rect)
);

fn main() {

}
