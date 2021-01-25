use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut w = String::with_capacity(31);
    stdin.read_line(&mut w).expect("read w");
    let w = w.trim();

    let answer = solve(w);

    println!("{}", answer);
}

fn solve(w: &str) -> String {
    w.chars()
        .filter(|&c| !is_vowel(c))
        .collect()
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'i' | 'u' | 'e' | 'o' => true,
        _ => false,
    }
}
