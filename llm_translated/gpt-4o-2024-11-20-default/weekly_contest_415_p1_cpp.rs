// Problem: Weekly Contest 415 Problem 1
use std::io::{self, Write};
use std::vec::Vec;

// Struct to represent the solution
struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = (nums.len() - 2) as i32;
        let mut xor_all = n ^ (n + 1);

        // Compute the XOR of all indices and elements in `nums`
        for (i, &num) in nums.iter().enumerate() {
            xor_all ^= i as i32 ^ num;
        }

        // Find the rightmost set bit (least significant bit that's `1`) in xor_all
        let shift = xor_all.trailing_zeros();

        // Initialize the answer to hold two values
        let mut ans = vec![0, 0];

        // Divide numbers into two groups based on the `shift` bit and XOR each group
        for (i, &num) in nums.iter().enumerate() {
            if i as i32 <= n {
                ans[((i >> shift) & 1) as usize] ^= i as i32;
            }
            ans[((num >> shift) & 1) as usize] ^= num;
        }

        ans
    }
}

fn main() {
    let mut input = String::new();

    // Read the first line to determine the number of elements (`numSize`)
    io::stdin().read_line(&mut input).expect("Failed to read number of elements");
    let num_size: usize = input.trim().parse().expect("Expected a valid integer") + 2;

    // Read the second line containing the elements of the vector
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read vector elements");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Expected a valid integer"))
        .collect();

    // Ensure the number of input elements matches the expected size
    assert_eq!(nums.len(), num_size);

    // Get the solution and compute sneaky numbers using the algorithm
    let solution = Solution;
    let result = solution.get_sneaky_numbers(nums);

    // Print the result in the specified format
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{} {}", result[0], result[1]).expect("Failed to write output");
}