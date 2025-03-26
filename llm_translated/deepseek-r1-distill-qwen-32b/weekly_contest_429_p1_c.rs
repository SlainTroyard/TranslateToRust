// Problem: Weekly Contest 429 Problem 1
use std::io;

fn minimum_operations(nums: &[i32]) -> i32 {
    let mut counts = [0; 101]; // Since nums[i] is between 1 and 100 inclusive
    let n = nums.len();
    
    for i in (0..n).rev() {
        let num = nums[i] as usize;
        counts[num] += 1;
        if counts[num] > 1 {
            return (i as i32 + 3) / 3;
        }
    }
    0
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        println!("0");
        return;
    }
    
    let n: usize = tokens[0].parse().expect("Invalid input for n");
    if tokens.len() < n + 1 {
        panic!("Insufficient numbers provided");
    }
    
    let nums: Vec<i32> = tokens[1..n+1]
        .iter()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    
    let result = minimum_operations(&nums);
    println!("{}", result);
}