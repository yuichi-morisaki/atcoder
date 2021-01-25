use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::with_capacity(6 * 6);
    stdin.read_line(&mut buf).expect("read buf");
    let v = buf.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let answer = solve(&v);

    println!("{}", answer);
}

fn solve(v: &Vec<i32>) -> f64 {
    let mut iter = v.iter();
    let x_a = iter.next().unwrap();
    let y_a = iter.next().unwrap();
    let x_b = iter.next().unwrap();
    let y_b = iter.next().unwrap();
    let x_c = iter.next().unwrap();
    let y_c = iter.next().unwrap();

    let det = (x_a - x_c) * (y_b - y_c) - (x_b - x_c) * (y_a - y_c);

    det.abs() as f64 / 2.0
}
