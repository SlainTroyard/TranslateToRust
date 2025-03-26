// Problem: Weekly Contest 431 Problem 2 in Rust

use std::io;

// Main solution function
fn calculate_score(s: &str) -> i64 {
    let len = s.len();
    let mut stacks: Vec<Vec<i32>> = vec![Vec::new(); 26]; // One stack for each letter, using Vec<i32> directly

    let mut ans: i64 = 0;

    for i in 0..len {
        let c_index = (s.as_bytes()[i] - b'a') as usize; // Calculate index 0-25 for char 'a' to 'z'
        let opposite_index = 25 - c_index;

        if !stacks[opposite_index].is_empty() {
            // Found a match - calculate score and pop
            if let Some(top_val) = stacks[opposite_index].pop() {
                ans += (i as i32 - top_val) as i64;
            }
        } else {
            // No match - push current position
            stacks[c_index].push(i as i32);
        }
    }

    // No need to explicitly free stacks in Rust

    ans
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim(); // Remove trailing newline

    println!("{}", calculate_score(s));
}