use std::collections::BTreeSet;

fn main() {
    let mut x1 = 0;
    let mut x2 = 65536;
    let mut x3 = 6237469;

    let mut fs = BTreeSet::<i32>::new();

    let mut last_added = 0;
    loop {
        while x2 >= 256 {
            x1 = x2 / 256;
            x2 = x1;
            x3 = (((x3 + (255 & x1)) & 16777215) * 65899) & 16777215;
        }

        println!("{} {}", x1, x3);
        if fs.contains(&x3) {
            break;
        }
        fs.insert(x3);
        last_added = x3;

        x2 = x3 | 65536;
        x3 = 1099159;
        x3 = (((x3 + (255 & x2)) & 16777215) * 65899) & 16777215;
    }
    println!("ans: {}", last_added);
}
