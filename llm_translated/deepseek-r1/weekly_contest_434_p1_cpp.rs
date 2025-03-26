use std::io::{self, Read};

struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            0
        } else {
            // Return number of elements minus 1 if sum is even
            nums.len() as i32 - 1
        }
    }
}

fn main() {
    // Read entire input and split into whitespace-separated tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    
    // Parse first token as number of elements
    let n: usize = tokens.next()
        .expect("Missing input")
        .parse()
        .expect("Invalid number format");
    
    // Parse next n tokens as integers
    let nums: Vec<i32> = tokens.take(n)
        .map(|s| s.parse().expect("Invalid integer format"))
        .collect();
    
    // Calculate and print result
    println!("{}", Solution::count_partitions(nums));
}