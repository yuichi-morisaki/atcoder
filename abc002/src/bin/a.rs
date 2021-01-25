use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut x = Vec::with_capacity(11);
    stdin.read_until(b' ', &mut x).expect("read x");
    let x = std::str::from_utf8(&x).expect("x from utf8");
    let x: u32 = x.trim_end().parse().expect("parse x");

    let mut y = String::with_capacity(11);
    stdin.read_line(&mut y).expect("read y");
    let y: u32 = y.trim_end().parse().expect("parse y");

    let answer = if x > y { x } else { y };

    println!("{}", answer);
}
