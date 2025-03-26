use std::io::{self, Read};

fn calculate_score(s: &str) -> i64 {
    let len = s.len();
    // Initialize 26 stacks, each with capacity to handle the maximum possible elements (string length)
    let mut stacks: [Vec<i32>; 26] = std::array::from_fn(|_| Vec::with_capacity(len));
    let mut ans = 0i64;

    for (i, ch) in s.chars().enumerate() {
        let c = (ch as u8 - b'a') as usize;
        let complement = 25 - c;

        // Check if the complement stack has elements
        if let Some(&top) = stacks[complement].last() {
            ans += i as i64 - top as i64;
            stacks[complement].pop();
        } else {
            // Push current index to the current character's stack
            stacks[c].push(i as i32);
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim(); // Trim whitespace to match C's scanf("%s") behavior
    println!("{}", calculate_score(s));
}