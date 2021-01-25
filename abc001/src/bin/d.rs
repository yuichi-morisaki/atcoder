use std::fmt;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut n = String::with_capacity(6);
    stdin.read_line(&mut n).expect("read n");
    let n: usize = n.trim_end().parse().expect("parse n");

    let mut v = Vec::with_capacity(n);

    for _ in 0..n {
        let mut line = String::with_capacity(10);
        stdin.read_line(&mut line).expect("read line");
        let bytes = line.as_bytes();
        let start = Time::new(&bytes[0..4]);
        let end = Time::new(&bytes[5..9]);
        v.push((start, end));
    }

    let answer = solve(v);

    for (start, end) in answer.iter() {
        println!("{}-{}", start, end);
    }
}

fn solve(mut v: Vec<(Time, Time)>) -> Vec<(Time, Time)> {
    v.sort();

    let mut answer = Vec::with_capacity(v.len());

    let mut prev_start = None;
    let mut prev_end = None;

    for (mut start, mut end) in v.into_iter() {
        start.round_down();
        end.round_up();

        if let (Some(ps), Some(pe)) = (prev_start, prev_end) {
            if pe < start {
                answer.push((ps, pe));
                prev_start = Some(start);
                prev_end = Some(end);
                continue;
            }
            if pe < end {
                prev_end = Some(end);
            }
        }
        else {
            prev_start = Some(start);
            prev_end = Some(end);
        }
    }

    if let (Some(ps), Some(pe)) = (prev_start, prev_end) {
        answer.push((ps, pe));
    }

    answer
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Time {
    minutes: u32,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let h = self.minutes / 60;
        let m = self.minutes % 60;

        write!(f, "{:02}{:02}", h, m)
    }
}

impl Time {
    pub fn new(t: &[u8]) -> Time {
        let h: u32 = std::str::from_utf8(&t[0..2])
            .expect("Time::new - str from hour")
            .parse()
            .expect("Time::new - parse hour");
        let m: u32 = std::str::from_utf8(&t[2..4])
            .expect("Time::new - str from minute")
            .parse()
            .expect("Time::new - parse minute");

        let minutes = 60 * h + m;

        Time { minutes }
    }

    pub fn round_down(&mut self) {
        let r = self.minutes % 10;

        self.minutes -= r;

        if r >= 5 {
            self.minutes += 5;
        }
    }

    pub fn round_up(&mut self) {
        let r = self.minutes % 10;

        if 0 < r && r < 5 {
            self.minutes += 5 - r;
        }
        else if 5 < r {
            self.minutes += 10 - r;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_new() {
        let v = b"0000";
        let t = Time::new(&v.to_vec());
        assert_eq!(t.minutes, 0);

        let v = b"1155";
        let t = Time::new(&v.to_vec());
        assert_eq!(t.minutes, 715);

        let v = b"2400";
        let t = Time::new(&v.to_vec());
        assert_eq!(t.minutes, 1440);
    }

    #[test]
    fn round_down() {
        let v = b"1200";
        let mut t = Time::new(&v.to_vec());
        t.round_down();
        assert_eq!(t.minutes, 720);

        let v = b"1201";
        let mut t = Time::new(&v.to_vec());
        t.round_down();
        assert_eq!(t.minutes, 720);

        let v = b"1204";
        let mut t = Time::new(&v.to_vec());
        t.round_down();
        assert_eq!(t.minutes, 720);

        let v = b"1205";
        let mut t = Time::new(&v.to_vec());
        t.round_down();
        assert_eq!(t.minutes, 725);

        let v = b"1206";
        let mut t = Time::new(&v.to_vec());
        t.round_down();
        assert_eq!(t.minutes, 725);

        let v = b"1209";
        let mut t = Time::new(&v.to_vec());
        t.round_down();
        assert_eq!(t.minutes, 725);
    }

    #[test]
    fn round_up_1() {
        let v = b"1200";
        let mut t = Time::new(&v.to_vec());
        t.round_up();
        assert_eq!(t.minutes, 720);

        let v = b"1201";
        let mut t = Time::new(&v.to_vec());
        t.round_up();
        assert_eq!(t.minutes, 725);

        let v = b"1204";
        let mut t = Time::new(&v.to_vec());
        t.round_up();
        assert_eq!(t.minutes, 725);

        let v = b"1205";
        let mut t = Time::new(&v.to_vec());
        t.round_up();
        assert_eq!(t.minutes, 725);

        let v = b"1206";
        let mut t = Time::new(&v.to_vec());
        t.round_up();
        assert_eq!(t.minutes, 730);

        let v = b"1209";
        let mut t = Time::new(&v.to_vec());
        t.round_up();
        assert_eq!(t.minutes, 730);
    }

    #[test]
    fn round_up_2() {
        let v = b"1250";
        let mut t = Time::new(&v.to_vec());
        t.round_up();
        assert_eq!(t.minutes, 770);

        let v = b"1251";
        let mut t = Time::new(&v.to_vec());
        t.round_up();
        assert_eq!(t.minutes, 775);

        let v = b"1254";
        let mut t = Time::new(&v.to_vec());
        t.round_up();
        assert_eq!(t.minutes, 775);

        let v = b"1255";
        let mut t = Time::new(&v.to_vec());
        t.round_up();
        assert_eq!(t.minutes, 775);

        let v = b"1256";
        let mut t = Time::new(&v.to_vec());
        t.round_up();
        assert_eq!(t.minutes, 780);

        let v = b"1259";
        let mut t = Time::new(&v.to_vec());
        t.round_up();
        assert_eq!(t.minutes, 780);
    }
}
