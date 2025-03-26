use std::collections::HashMap;
use std::io::{self, Read};

struct Solution;

impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        let mut sum = 0;

        // Populate the frequency map and calculate the total sum
        for &num in &nums {
            *counts.entry(num).or_insert(0) += 1;
            sum += num;
        }

        // Collect unique numbers and sort in descending order
        let mut candidates: Vec<i32> = counts.keys().cloned().collect();
        candidates.sort_by(|a, b| b.cmp(a));

        // Check each candidate as potential outlier
        for &candidate in &candidates {
            let sum_remaining = sum - candidate;
            if sum_remaining % 2 != 0 {
                continue;
            }
            let d = sum_remaining / 2;

            // Check if the required pair exists with proper count
            if let Some(&count) = counts.get(&d) {
                if d != candidate || count > 1 {
                    return candidate;
                }
            }
        }

        -1
    }
}

fn main() {
    // Read all input at once to handle any whitespace-separated format
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    // Parse number of elements and the array values
    let n: i32 = tokens.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = tokens
        .take(n as usize)
        .map(|s| s.parse().unwrap())
        .collect();

    // Compute and print the result
    println!("{}", Solution::get_largest_outlier(nums));
}