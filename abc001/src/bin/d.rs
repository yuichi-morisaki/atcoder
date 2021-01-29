use std::io::Read;

const BUF_SIZE: usize = (5 + 1) + (4 + 1 + 4 + 1) * 30000;

fn main() {
    let mut buf = String::with_capacity(BUF_SIZE);
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let (n, v) = read(&buf);
    let v = solve(n, v);
    write(v);
}

fn read(buf: &str) -> (usize, Vec<(i32, i32)>) {
    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    assert!(1 <= n && n <= 30_000);

    let mut v = Vec::with_capacity(n);

    for _ in 0..n {
        let line = iter.next().unwrap();
        let mut iter = line.split('-');

        let s: i32 = iter.next().unwrap().parse().unwrap();
        let sh = s / 100;
        let sm = s % 100;
        assert!(0 <= sh && sh <= 24);
        assert!(0 <= sm && sm <= 59);

        let e: i32 = iter.next().unwrap().parse().unwrap();
        let eh = e / 100;
        let em = e % 100;
        assert!(0 <= eh && eh <= 24);
        assert!(0 <= em && em <= 59);

        v.push((sh * 60 + sm, eh * 60 + em));
    }

    (n, v)
}

fn solve(_n: usize, mut v: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    v.sort();

    let mut answer = Vec::with_capacity(v.len());

    let mut iter = v.into_iter();
    let (mut prev_start, mut prev_end) = iter.next()
        .map(|(start, end)| (round_down(start), round_up(end)))
        .unwrap();

    loop {
        if let Some((start, end)) = iter.next()
                .map(|(start, end)| (round_down(start), round_up(end))) {
            if prev_end < start {
                answer.push((prev_start, prev_end));
                prev_start = start;
                prev_end = end;
            } else {
                if prev_end < end {
                    prev_end = end;
                }
            }
        } else {
            break;
        }
    }

    answer.push((prev_start, prev_end));

    answer
}

fn round_down(m: i32) -> i32 {
    match m % 10 {
        0 | 5 => m,
        r if 0 < r && r < 5 => m - r,
        r if 5 < r && r < 10 => m - r + 5,
        _ => panic!("invalid input m: {}", m),
    }
}

fn round_up(m: i32) -> i32 {
    match m % 10 {
        0 | 5  => m,
        r if 0 < r && r < 5 => m - r + 5,
        r if 5 < r && r < 10 => m - r + 10,
        _ => panic!("invalid input m: {}", m),
    }
}

fn write(v: Vec<(i32, i32)>) {
    for &(start, end) in v.iter() {
        println!(
            "{:02}{:02}-{:02}{:02}",
            start / 60, start % 60, end / 60, end % 60
        );
    }
}
