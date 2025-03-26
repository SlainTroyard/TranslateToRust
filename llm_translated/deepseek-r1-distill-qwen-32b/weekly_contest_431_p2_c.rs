// Problem: Weekly Contest 431 Problem 2
// Solution translated to Rust with idiomatic practices

use std::io;

// Each stack is represented by a Vec<i32>
type Stack = Vec<i32>;

/// Calculate the score based on matching characters in the string
fn calculate_score(s: &str) -> i64 {
    let len = s.len();
    let mut stacks: Vec<Stack> = vec![Vec::new(); 26];
    let mut ans: i64 = 0;

    for (i, c) in s.chars().enumerate() {
        let c_index = c as usize - 'a' as usize;
        let reverse_index = 25 - c_index;

        // Check if the reverse stack is not empty
        if !stacks[reverse_index].is_empty() {
            // Pop the top element and calculate the score
            let top = stacks[reverse_index].pop().unwrap();
            ans += (i as i64) - (top as i64);
        } else {
            // Push current index onto the current stack
            stacks[c_index].push(i as i32);
        }
    }

    ans
}

fn main() {
    // Read input string
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim(); // Remove any trailing newline or whitespace

    // Calculate and print the result
    println!("{}", calculate_score(s));
}