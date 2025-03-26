use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn has_increasing_subarrays(nums: &Vec<i32>, k: i32) -> bool {
        let k_usize = k as usize;

        // Define a closure to check if a subarray is strictly increasing
        let mono = |idx: usize| -> bool {
            for i in idx..(idx + k_usize - 1) {
                if nums[i] >= nums[i + 1] {
                    return false;
                }
            }
            true
        };

        // Iterate through possible starting indices of the first subarray
        for idx in 0..=(nums.len() - 2 * k_usize) {
            // Check if both subarrays starting at idx and idx + k are strictly increasing
            if mono(idx) && mono(idx + k_usize) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    // Read the size of the array
    let n: usize = lines.next().unwrap().parse().unwrap();

    // Read the elements of the array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read the length of the subarray
    let k: i32 = lines.next().unwrap().parse().unwrap();

    // Create a Solution object and call the function
    let sol = Solution {};
    if sol.has_increasing_subarrays(&nums, k)) {
        println!("true");
    } else {
        println!("false");
    }
}