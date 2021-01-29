use std::io::Read;

const BUF_SIZE: usize = (4 + 1) * 2;

fn main() {
    let mut buf = String::with_capacity(BUF_SIZE);
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let (h1, h2) = read(&buf);
    let diff = solve(h1, h2);
    write(diff);
}

fn read(buf: &str) -> (i32, i32) {
    let mut iter = buf.split_whitespace();

    let h1: i32 = iter.next().unwrap().parse().unwrap();
    assert!(0 <= h1 && h1 <= 2000);

    let h2: i32 = iter.next().unwrap().parse().unwrap();
    assert!(0 <= h2 && h2 <= 2000);

    (h1, h2)
}

fn solve(h1: i32, h2: i32) -> i32 {
    h1 - h2
}

fn write(diff: i32) {
    println!("{}", diff);
}
