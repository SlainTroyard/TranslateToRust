use std::io;
use std::str::SplitWhitespace;

struct Solution {}

impl Solution {
    pub fn has_increasing_subarrays(&self, nums: &Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 * k as usize {
            return false;
        }
        if k <= 0 {
            return false; // Handle invalid k if needed, based on problem constraints
        }
        let k_usize = k as usize;

        // Closure to check if a subarray starting at idx of length k is increasing
        let mono = |idx: usize| -> bool {
            if idx + k_usize > nums.len() {
                return false; // Index out of bounds, should not happen in this logic but good to have
            }
            for i in 0..(k_usize - 1) {
                if nums[idx + i] >= nums[idx + i + 1] {
                    return false;
                }
            }
            return true;
        };

        for idx in 0..=(nums.len() - 2 * k_usize) {
            if mono(idx) && mono(idx + k_usize) {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().trim().parse().expect("n is not an integer");

    let mut nums = Vec::with_capacity(n);
    let mut input_nums = String::new();
    io::stdin().read_line(&mut input_nums).expect("Failed to read line");
    let nums_iter = input_nums.split_whitespace();
    for s in nums_iter.take(n) {
        nums.push(s.trim().parse().expect("nums element is not an integer"));
    }

    let mut input_k = String::new();
    io::stdin().read_line(&mut input_k).expect("Failed to read line");
    let k: i32 = input_k.trim().parse().expect("k is not an integer");

    let sol = Solution {};
    if sol.has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }
}