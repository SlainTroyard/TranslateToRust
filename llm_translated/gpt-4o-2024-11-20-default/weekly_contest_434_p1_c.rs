use std::io::{self, Write};

fn count_partitions(nums: &[i32]) -> i32 {
    let mut count = 0;
    let nums_size = nums.len();
    
    // Iterate through possible partitions
    for i in 0..nums_size - 1 {
        let left_sum: i32 = nums[..=i].iter().sum();
        let right_sum: i32 = nums[i + 1..].iter().sum();
        
        // Check if the difference is even
        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }
    }
    
    count
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    // Read the first line for `n`
    stdin.read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse input as integer");
    
    // Read the second line for the array of numbers
    input.clear();
    stdin.read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse input as integer"))
        .collect();
    
    // Ensure the input length matches the expected size
    assert_eq!(nums.len(), n, "Input array size does not match n");
    
    // Calculate and print the result
    let result = count_partitions(&nums);
    println!("{}", result);
}