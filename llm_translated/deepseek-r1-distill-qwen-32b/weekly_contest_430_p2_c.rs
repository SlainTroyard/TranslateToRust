// Problem: Weekly Contest 430 Problem 2
// This Rust code is a translation of the provided C code, maintaining the exact logic and input/output format.

use std::io;

/// Finds the lexicographically largest substring of a specific length and returns it.
/// 
/// # Arguments
/// * `word` - The input string to process.
/// * `num_friends` - Determines the length of the substring to find.
/// 
/// # Returns
/// The largest substring of the calculated length.
fn answer_string(word: &str, num_friends: i32) -> &str {
    if num_friends == 1 {
        return word;
    }

    let len = word.len();
    let n = len - (num_friends as usize - 1);
    
    // If the calculated length is non-positive, return an empty string
    if n <= 0 {
        return "";
    }

    let max_i = len - n;
    if max_i < 0 {
        return "";
    }

    let mut ans = 0;
    for i in 0..=max_i {
        let substr_i = &word[i..i + n];
        let substr_ans = &word[ans..ans + n];
        if substr_i > substr_ans {
            ans = i;
        }
    }

    &word[ans..ans + n]
}

fn main() {
    // Read all input at once
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    // Split input into parts
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    // Extract word and number of friends
    let word = parts[0];
    let num_friends = parts[1].parse::<i32>().unwrap();
    
    // Get the result
    let result = answer_string(word, num_friends);
    
    // Print the result
    println!("{}", result);
}