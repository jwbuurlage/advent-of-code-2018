use std::io;

pub fn read_all<T: std::str::FromStr>() -> Vec<T> {
    let mut xs = vec!();
    loop {
        let mut a_str = String::new();
        match io::stdin().read_line(&mut a_str) {
            Ok(x) => if x == 0 { break },
            Err(_) => break
        }
        match a_str.trim().parse::<T>() {
            Ok(x) => xs.push(x),
            Err(_) => break
        }
    }

    return xs
}
