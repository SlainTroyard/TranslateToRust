use std::cmp::max;
use std::io;

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
    let mut input = String::new();
    
    // Read the input string
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim();
    
    // Read the integer k
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: usize = input.trim().parse().expect("Please type a number!");
    
    // Call the answer_string function
    let result = answer_string(s, k);
    
    // Print the result
    println!("{}", result);
}