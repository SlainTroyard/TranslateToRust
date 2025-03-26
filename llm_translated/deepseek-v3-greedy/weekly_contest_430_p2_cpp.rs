use std::cmp::max;
use std::io::{self, Write};

fn answer_string(s: &str, k: usize) -> String {
    if k == 1 {
        return s.to_string();
    }
    let n = s.len();
    let mut ans = String::new();
    for i in 0..n {
        let substr = &s[i..n - max(k - 1, i)];
        if substr > ans {
            ans = substr.to_string();
        }
    }
    ans
}

fn main() {
    // Read input string
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim(); // Remove any trailing newline

    // Read input k
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read line");
    let k: usize = k_input.trim().parse().expect("Please enter a valid number");

    // Call answer_string function
    let result = answer_string(s, k);

    // Print the result
    println!("{}", result);
}