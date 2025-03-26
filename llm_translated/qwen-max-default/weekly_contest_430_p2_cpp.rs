// Problem: Weekly Contest 430 Problem 2

use std::io;

fn answer_string(s: &str, k: usize) -> String {
    if k == 1 {
        return s.to_string();
    }
    let n = s.len();
    let mut ans = String::new();
    for i in 0..n {
        let substr = &s[i..(n - (k - 1).max(i))];
        if substr > &ans {
            ans = substr.to_string();
        }
    }
    ans
}

fn main() {
    // Input string and k
    let mut s = String::new();
    let mut k_str = String::new();

    println!("Enter the string:");
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s = s.trim().to_string(); // Remove any trailing newline or whitespace

    println!("Enter the value of k:");
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: usize = k_str.trim().parse().expect("Please type a number!");

    // Call answer_string function
    let result = answer_string(&s, k);

    // Print the result
    println!("{}", result);
}