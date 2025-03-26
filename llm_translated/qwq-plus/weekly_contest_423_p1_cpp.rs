use std::io;

struct Solution;

impl Solution {
    // Helper function to check if a subarray starting at 'start' with 'length' elements is strictly increasing
    fn is_increasing_subarray(nums: &[i32], start: usize, length: usize) -> bool {
        for i in start..start + length - 1 {
            if nums[i] >= nums[i + 1] {
                return false;
            }
        }
        true
    }

    // Main method to check for the existence of two non-overlapping increasing subarrays
    pub fn has_increasing_subarrays(nums: &[i32], k: i32) -> bool {
        let k_usize = k as usize;
        if k_usize == 0 || nums.len() < 2 * k_usize {
            return false;
        }

        for idx in 0..=(nums.len() - 2 * k_usize) {
            if Self::is_increasing_subarray(nums, idx, k_usize)
                && Self::is_increasing_subarray(
                    nums,
                    idx + k_usize,
                    k_usize,
                ) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid number"));

    let n = tokens.next().expect("n not found");
    let mut nums = Vec::new();
    for _ in 0..n {
        nums.push(tokens.next().expect("Missing number"));
    }
    let k = tokens.next().expect("k not found");

    let solution = Solution;
    let result = solution.has_increasing_subarrays(&nums, k);
    println!("{}", if result { "true" } else { "false" });
}