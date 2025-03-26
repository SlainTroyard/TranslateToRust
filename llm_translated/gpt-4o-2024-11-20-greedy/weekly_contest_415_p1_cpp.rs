// Problem: Weekly Contest 415 Problem 1
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32 - 2;
        let mut xor_all = n ^ (n + 1);

        // Perform the XOR operations on the entire range and nums
        for i in 0..nums.len() as i32 {
            xor_all ^= i ^ nums[i as usize];
        }

        // Determine the least significant set bit (shift)
        let shift = xor_all.trailing_zeros() as usize;

        let mut ans = vec![0; 2];
        for i in 0..nums.len() as i32 {
            if i < n {
                ans[(i >> shift) as usize & 1] ^= i;
            }
            ans[(nums[i as usize] >> shift) as usize & 1] ^= nums[i as usize];
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements to process (excluding the extra 2 implied in the question)
    let num_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse numSize");

    // The problem implies we expect num_size + 2 numbers in total
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Failed to parse num"))
        .collect();

    assert_eq!(nums.len(), num_size + 2, "Input size does not match expected size");

    // Compute the result
    let solution = Solution;
    let result = solution.get_sneaky_numbers(nums);

    // Print the result
    for num in result {
        print!("{} ", num);
    }
    println!();
}