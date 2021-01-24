use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        w: Chars,
    }

    let answer = solve(w);

    println!("{}", answer);
}

fn solve(w: Vec<char>) -> String {
    w.into_iter()
        .filter(|c| !is_vowel(*c))
        .collect()
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'i' | 'u' | 'e' | 'o' => true,
        _ => false,
    }
}
