use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
    }

    println!("{}", if x > y { x } else { y });
}
