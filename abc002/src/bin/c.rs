use proconio::input;

fn main() {
    input! {
        x_a: i32,
        y_a: i32,
        x_b: i32,
        y_b: i32,
        x_c: i32,
        y_c: i32,
    }

    let answer = solve(x_a, y_a, x_b, y_b, x_c, y_c);

    println!("{}", answer);
}

fn solve(x_a: i32, y_a: i32, x_b: i32, y_b: i32, x_c: i32, y_c: i32) -> f64 {
    let det = (x_a - x_c) * (y_b - y_c) - (x_b - x_c) * (y_a - y_c);

    det.abs() as f64 / 2.0
}
