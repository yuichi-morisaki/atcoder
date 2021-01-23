use proconio::input;

fn main() {
    input! {
        m: u32,
    }

    let vv = solve(m);
    println!("{:02}", vv);
}

fn solve(m: u32) -> u32 {
    if m < 100 {
        0
    } else if 100 <= m && m <= 5_000 {
        m * 10 / 1000
    } else if 6_000 <= m && m <= 30_000 {
        m / 1000 + 50
    } else if 35_000 <= m && m <= 70_000 {
        (m / 1000 - 30) / 5 + 80
    } else if 70_000 < m {
        89
    } else {
        panic!("invalid input: {}", m);
    }
}
