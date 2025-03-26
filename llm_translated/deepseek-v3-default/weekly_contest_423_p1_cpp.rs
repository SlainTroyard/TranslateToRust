use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Function to check if there are two increasing subarrays of length `k` in the given vector
    pub fn has_increasing_subarrays(nums: &Vec<i32>, k: usize) -> bool {
        // Helper function to check if a subarray starting at `idx` is monotonically increasing
        let mono = |idx: usize| -> bool {
            for i in idx..idx + k - 1 {
                if nums[i] >= nums[i + 1] {
                    return false;
                }
            }
            true
        };

        // Iterate over possible starting indices for the first subarray
        for idx in 0..=nums.len() - 2 * k {
            if mono(idx) && mono(idx + k) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read the elements of the array
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read the length of the subarray
    let k: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Create a Solution object and call the function
    let sol = Solution;
    if sol.has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }
}