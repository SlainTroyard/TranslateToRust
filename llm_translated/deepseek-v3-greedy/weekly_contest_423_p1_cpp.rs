use std::io::{self, BufRead};

fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
    // Helper function to check if a subarray starting at `idx` is monotonically increasing
    let mono = |idx: usize| -> bool {
        for i in idx..idx + k - 1 {
            if nums[i] >= nums[i + 1] {
                return false;
            }
        }
        true
    };

    // Iterate over all possible starting indices for the first subarray
    for idx in 0..=nums.len() - 2 * k {
        if mono(idx) && mono(idx + k) {
            return true;
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the elements of the array
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read the length of the subarray
    let k: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Check if there are two increasing subarrays of length `k`
    if has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }
}