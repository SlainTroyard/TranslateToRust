// Problem: Weekly Contest 430 Problem 2
use std::io;

fn answer_string(s: &str, k: usize) -> String {
    if k == 1 {
        return s.to_string();
    }
    let n = s.len();
    let mut ans = String::new();
    for i in 0..n {
        let substr = &s[i..(i + n - (k - 1).max(i))];
        if substr > &ans {
            ans = substr.to_string();
        }
    }
    ans
}

fn main() {
    // Input string and k
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: usize = input.trim().parse().expect("Please type a number!");

    // Call answer_string function
    let result = answer_string(s, k);

    // Print the result
    println!("{}", result);
}