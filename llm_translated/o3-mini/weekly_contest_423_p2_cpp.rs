use std::cmp::{max, min};
use std::io::{self, BufRead};

/// Computes the maximum length based on a special partitioning of 
/// increasing subarrays as described in the problem statement.
/// 
/// We traverse through the vector `nums`, tracking the length of the current
/// increasing sequence (`curr`) and the previous sequence length (`prev`).
/// On every step, if the current number is larger than the previous, we increment
/// `curr`. Otherwise, we set `prev` to `curr` and reset `curr` to 1. 
/// Then, we update the answer using:
///   ans = max(ans, max(curr / 2, min(prev, curr)))
fn max_increasing_subarrays(nums: &[i32]) -> i32 {
    let mut prev = 0;
    let mut curr = 1;  // current increasing subarray length
    let mut ans = 0;

    // Traverse the nums array starting from index 1
    for i in 1..nums.len() {
        if nums[i - 1] < nums[i] {
            curr += 1;  // Increase the length of the current increasing subarray
        } else {
            prev = curr; // Update the previous subarray length
            curr = 1;    // Reset the current subarray length
        }
        // Update the answer by considering both the current and previous subarray lengths
        ans = max(ans, max(curr / 2, min(prev, curr)));
    }
    ans
}

fn main() {
    // Use a buffered reader for efficient input handling
    let stdin = io::stdin();
    let reader = stdin.lock();
    // Read all lines from standard input into a single string
    let input: String = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect::<Vec<String>>()
        .join(" ");
    
    // Split the input into tokens based on whitespace
    let tokens: Vec<&str> = input.split_whitespace().collect();
    
    // Create an iterator over tokens for parsing
    let mut iter = tokens.iter();
    
    // Read the size of the array
    let n: usize = iter
        .next()
        .expect("Expected array size")
        .parse()
        .expect("Failed to parse array size as integer");
    
    // Read the array elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num = iter
            .next()
            .expect("Expected more numbers")
            .parse::<i32>()
            .expect("Failed to parse number as integer");
        nums.push(num);
    }
    
    // Call the function to compute the answer
    let result = max_increasing_subarrays(&nums);
    
    // Output the result exactly as expected: a single integer followed by a newline
    println!("{}", result);
}