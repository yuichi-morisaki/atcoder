use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut h1 = String::with_capacity(5);
    stdin.read_line(&mut h1).expect("read h1");
    let h1: i32 = h1.trim_end().parse().expect("parse h1");

    let mut h2 = String::with_capacity(5);
    stdin.read_line(&mut h2).expect("read h2");
    let h2: i32 = h2.trim_end().parse().expect("parse h2");

    let answer = h1 - h2;

    println!("{}", answer);
}
