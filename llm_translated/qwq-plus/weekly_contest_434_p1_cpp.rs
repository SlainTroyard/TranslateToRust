use std::io;

struct Solution;

impl Solution {
    fn count_partitions(nums: &[i32]) -> i32 {
        let total_sum: i32 = nums.iter().sum();
        if total_sum % 2 != 0 {
            0
        } else {
            (nums.len() - 1) as i32
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    // Parse all integers from input
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();
    
    // Extract n and the nums array
    let n = numbers[0] as usize;
    let nums_slice = &numbers[1..=n]; // Slice from index 1 to n (inclusive)
    
    // Compute and print the result
    let result = Solution::count_partitions(nums_slice);
    println!("{}", result);
}