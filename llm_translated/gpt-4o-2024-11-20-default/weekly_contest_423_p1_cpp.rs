use std::io;

struct Solution;

impl Solution {
    fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
        // Closure to check if a subarray is strictly increasing
        let is_increasing = |idx: usize| -> bool {
            for i in idx..(idx + k - 1) {
                if nums[i] >= nums[i + 1] {
                    return false;
                }
            }
            true
        };

        // Check two subarrays of length k
        for idx in 0..=nums.len().saturating_sub(2 * k) {
            if is_increasing(idx) && is_increasing(idx + k) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();

    // Read the size of the array `n`
    stdin.read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid number");

    // Read the elements of the array `nums`
    input.clear();
    stdin.read_line(&mut input).expect("Failed to read input");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    // Read the length of the subarray `k`
    input.clear();
    stdin.read_line(&mut input).expect("Failed to read input");
    let k: usize = input.trim().parse().expect("Invalid number");

    // Create a Solution object and call the function
    let solution = Solution;
    if solution.has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }
}