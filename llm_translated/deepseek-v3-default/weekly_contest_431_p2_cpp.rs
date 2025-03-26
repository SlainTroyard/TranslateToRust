use std::collections::VecDeque;
use std::io;

fn calculate_score(s: &str) -> i64 {
    let mut stk: Vec<VecDeque<usize>> = vec![VecDeque::new(); 26];
    let mut ans: i64 = 0;

    for (i, ch) in s.chars().enumerate() {
        let c = (ch as u8 - b'a') as usize;
        if let Some(&top) = stk[25 - c].back() {
            ans += (i - top) as i64;
            stk[25 - c].pop_back();
        } else {
            stk[c].push_back(i);
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim(); // Remove any trailing newline characters

    let score = calculate_score(s);
    println!("{}", score);
}