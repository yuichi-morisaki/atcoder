use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut deg = Vec::with_capacity(5);
    stdin.read_until(b' ', &mut deg).expect("read deg");
    let deg = std::str::from_utf8(&deg).expect("deg from utf8");
    let deg: u32 = deg.trim_end().parse().expect("parse deg");

    let mut dis = String::with_capacity(6);
    stdin.read_line(&mut dis).expect("read dis");
    let dis: u32 = dis.trim_end().parse().expect("parse dis");

    let (dir, w) = solve(deg, dis);

    if w == 0 {
        println!("C 0");
    } else {
        println!("{} {}", dir, w);
    }
}

fn solve(deg: u32, dis: u32) -> (String, u32) {
    let dir = direction(deg);
    let wp = wind_power(dis);

    (dir, wp)
}

fn wind_power(dis: u32) -> u32 {
    let speed = (dis as f64 / 60.0 * 10.0).round() as u32;

    if speed <= 2 { // always 0 <= speed
        0
    } else if 3 <= speed && speed <= 15 {
        1
    } else if 16 <= speed && speed <= 33 {
        2
    } else if 34 <= speed && speed <= 54 {
        3
    } else if 55 <= speed && speed <= 79 {
        4
    } else if 80 <= speed && speed <= 107 {
        5
    } else if 108 <= speed && speed <= 138 {
        6
    } else if 139 <= speed && speed <= 171 {
        7
    } else if 172 <= speed && speed <= 207 {
        8
    } else if 208 <= speed && speed <= 244 {
        9
    } else if 245 <= speed && speed <= 284 {
        10
    } else if 285 <= speed && speed <= 326 {
        11
    } else { // 327 <= speed
        12
    }
}

fn direction(deg: u32) -> String {
    let deg = deg * 10;

    if 1125 <= deg && deg < 3375 {
        String::from("NNE")
    } else if 3375 <= deg && deg < 5625 {
        String::from("NE")
    } else if 5625 <= deg && deg < 7875 {
        String::from("ENE")
    } else if 7875 <= deg && deg < 10125 {
        String::from("E")
    } else if 10125 <= deg && deg < 12375 {
        String::from("ESE")
    } else if 12375 <= deg && deg < 14625 {
        String::from("SE")
    } else if 14625 <= deg && deg < 16875 {
        String::from("SSE")
    } else if 16875 <= deg && deg < 19125 {
        String::from("S")
    } else if 19125 <= deg && deg < 21375 {
        String::from("SSW")
    } else if 21375 <= deg && deg < 23625 {
        String::from("SW")
    } else if 23625 <= deg && deg < 25875 {
        String::from("WSW")
    } else if 25875 <= deg && deg < 28125 {
        String::from("W")
    } else if 28125 <= deg && deg < 30375 {
        String::from("WNW")
    } else if 30375 <= deg && deg < 32625 {
        String::from("NW")
    } else if 32625 <= deg && deg < 34875 {
        String::from("NNW")
    } else {
        String::from("N")
    }
}
