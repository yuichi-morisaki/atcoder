use std::io::Read;

const BUF_SIZE: usize = 6 + 1;

fn main() {
    let mut buf = String::with_capacity(BUF_SIZE);
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let m = read(&buf);
    let vv = solve(m);
    write(vv);
}

fn read(buf: &str) -> i32 {
    let m: i32 = buf.trim().parse().unwrap();
    assert!(0 <= m && m <= 100_000);

    m
}

fn solve(m: i32) -> i32 {
    let k = f64::from(m) / 1000.0;

    if k < 0.1 {
        return 0;
    }
    if 0.1 <= k && k <= 5.0 {
        let vv = k * 10.0;
        assert_eq!(vv.fract(), 0.0);
        return vv as i32;
    }
    if 6.0 <= k && k <= 30.0 {
        let vv = k + 50.0;
        assert_eq!(vv.fract(), 0.0);
        return vv as i32;
    }
    if 35.0 <= k && k <= 70.0 {
        let vv = (k - 30.0) / 5.0 + 80.0;
        assert_eq!(vv.fract(), 0.0);
        return vv as i32;
    }
    if 70.0 < k {
        return 89;
    }

    panic!("invalid input");
}

fn write(vv: i32) {
    println!("{:02}", vv);
}
