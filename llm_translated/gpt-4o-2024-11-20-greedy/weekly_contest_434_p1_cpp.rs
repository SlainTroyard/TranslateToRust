use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        // Calculate the sum of the elements in the vector
        let s: i32 = nums.iter().sum();
        // If the sum is odd, return 0; otherwise, return the size of the vector minus 1
        if s % 2 != 0 {
            0
        } else {
            nums.len() as i32 - 1
        }
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line contains the number of elements in the array
    let n: usize = lines
        .next()
        .expect("Expected input")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse number");

    // Second line contains the array elements
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input")
        .expect("Failed to read line")
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse number"))
        .collect();

    // Ensure the number of elements matches the input size
    assert_eq!(nums.len(), n, "Input size does not match the number of elements");

    // Create a solution instance and compute the result
    let sol = Solution;
    let result = sol.count_partitions(nums);

    // Print the result to stdout
    println!("{}", result);
}