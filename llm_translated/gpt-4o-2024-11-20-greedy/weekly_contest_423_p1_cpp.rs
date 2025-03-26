use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
        // Closure to check if a subarray of length `k` starting at `idx` is strictly increasing
        let mono = |idx: usize| -> bool {
            for i in idx..idx + k - 1 {
                if nums[i] >= nums[i + 1] {
                    return false;
                }
            }
            true
        };

        // Iterate over possible starting indices for the two subarrays
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
    let n: usize = lines
        .next()
        .expect("Expected input for array size")
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid input for array size");

    // Read the elements of the array
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input for array elements")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input for array element"))
        .collect();

    // Read the length of the subarray
    let k: usize = lines
        .next()
        .expect("Expected input for subarray length")
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid input for subarray length");

    // Ensure the input sizes are valid
    assert_eq!(nums.len(), n, "Array size does not match the input size");

    // Create a Solution object and call the function
    let sol = Solution;
    if sol.has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }
}