use std::io::{self, Read};

fn count_partitions(nums: &[i32]) -> i32 {
    let mut count = 0;
    // Iterate through all possible partition points (0 to len-2)
    for i in 0..nums.len().saturating_sub(1) {
        // Calculate sum of left partition using iterator and sum()
        let left_sum: i32 = nums[0..=i].iter().sum();
        // Calculate sum of right partition using iterator and sum()
        let right_sum: i32 = nums[i+1..].iter().sum();
        // Check if the difference is even
        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    // Read all input at once for efficient parsing
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    // Split input into whitespace-separated tokens
    let mut tokens = input.split_whitespace();
    
    // Parse first token as the array length
    let n: usize = tokens.next()
        .expect("Missing array length")
        .parse()
        .expect("Invalid array length");
    
    // Parse next n tokens as the array elements
    let nums: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().expect("Invalid integer in array"))
        .collect();
    
    // Verify we collected exactly n elements
    if nums.len() != n {
        panic!("Insufficient elements in input");
    }
    
    // Calculate and print the result
    println!("{}", count_partitions(&nums));
}