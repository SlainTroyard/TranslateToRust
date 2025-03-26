use std::io::{self, Read};

fn calculate_score(s: &str) -> i64 {
    // Initialize 26 stacks, each corresponding to a letter 'a' to 'z'
    let mut stacks: [Vec<usize>; 26] = Default::default();
    let mut ans = 0i64;

    for (i, c) in s.chars().enumerate() {
        let c_val = (c as u8 - b'a') as usize;
        let comp_index = 25 - c_val;  // Complementary character's stack index

        // Check if the complementary character's stack has elements
        if let Some(top) = stacks[comp_index].last().cloned() {
            // Calculate contribution to the answer and pop the index
            ans += i as i64 - top as i64;
            stacks[comp_index].pop();
        } else {
            // Push current index to the current character's stack
            stacks[c_val].push(i);
        }
    }

    ans
}

fn main() {
    // Read all input from stdin and split into whitespace-separated tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let s = input.split_whitespace().next().expect("No input provided");
    
    // Calculate and print the result
    println!("{}", calculate_score(s));
}