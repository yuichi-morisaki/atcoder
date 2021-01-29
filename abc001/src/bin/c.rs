use std::io::Read;

const BUF_SIZE: usize = (4 + 1) + (5 + 1);

fn main() {
    let mut buf = String::with_capacity(BUF_SIZE);
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let (deg, dis) = read(&buf);
    let (dir, w) = solve(deg, dis);
    write(dir, w);
}

fn read(buf: &str) -> (i32, i32) {
    let mut iter = buf.split_whitespace();

    let deg: i32 = iter.next().unwrap().parse().unwrap();
    assert!(0 <= deg && deg < 3600);

    let dis: i32 = iter.next().unwrap().parse().unwrap();
    assert!(0 <= dis && dis < 12000);

    (deg, dis)
}

fn solve(deg: i32, dis: i32) -> (usize, i32) {
    let deg = f64::from(deg) / 10.0;
    let dir = ((deg + 11.25).rem_euclid(360.0)).div_euclid(22.5) as usize;

    let speed = f64::from(dis) / 60.0;
    let speed = (speed * 10.0).round() / 10.0;
    let w = match speed {
        s if  0.0 <= s && s <=  0.2 =>  0,
        s if  0.3 <= s && s <=  1.5 =>  1,
        s if  1.6 <= s && s <=  3.3 =>  2,
        s if  3.4 <= s && s <=  5.4 =>  3,
        s if  5.5 <= s && s <=  7.9 =>  4,
        s if  8.0 <= s && s <= 10.7 =>  5,
        s if 10.8 <= s && s <= 13.8 =>  6,
        s if 13.9 <= s && s <= 17.1 =>  7,
        s if 17.2 <= s && s <= 20.7 =>  8,
        s if 20.8 <= s && s <= 24.4 =>  9,
        s if 24.5 <= s && s <= 28.4 => 10,
        s if 28.5 <= s && s <= 32.6 => 11,
        s if 32.7 <= s => 12,
        _ => panic!("invalid input"),
    };

    (dir, w)
}

fn write(dir: usize, w: i32) {
    let directions = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE",
        "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW",
    ];

    if w == 0 {
        println!("{} {}", "C", w);
    } else {
        println!("{} {}", directions[dir], w);
    }
}
