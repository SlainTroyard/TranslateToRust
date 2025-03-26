use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn get_sneaky_numbers(nums: &Vec<i32>) -> Vec<i32> {
        let n = nums.len() - 2;
        // XOR of all expected indices and values
        let mut xor_all = n as i32 ^ (n as i32 + 1);
        for i in 0..nums.len() {
            xor_all ^= i as i32 ^ nums[i];
        }
        
        // Find the rightmost set bit position
        // Rust equivalent of __builtin_ctz
        let shift = xor_all.trailing_zeros() as usize;

        let mut ans = vec![0, 0];
        for i in 0..nums.len() {
            if i < n {
                // Group indices based on the bit at position 'shift'
                ans[((i as i32) >> shift & 1) as usize] ^= i as i32;
            }
            // Group values based on the bit at position 'shift'
            ans[(nums[i] >> shift & 1) as usize] ^= nums[i];
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line to get numSize
    let mut num_size: usize = lines.next()
        .expect("Failed to read input")
        .expect("Failed to parse input")
        .trim()
        .parse()
        .expect("Failed to parse numSize");
    
    // Add 2 to numSize as in the original code
    num_size += 2;
    
    // Read the second line to get the numbers
    let nums: Vec<i32> = lines.next()
        .expect("Failed to read input")
        .expect("Failed to parse input")
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Ensure we have the correct number of elements
    assert_eq!(nums.len(), num_size, "Input size mismatch");
    
    let solution = Solution;
    let result = Solution::get_sneaky_numbers(&nums);
    
    // Print the result with the same format as the original code
    for i in 0..2 {
        print!("{} ", result[i]);
    }
}