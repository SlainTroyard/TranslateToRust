use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn get_sneaky_numbers(nums: &mut Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32 - 2;
        let mut xor_all = (n ^ (n + 1)) as i32;

        for i in 0..nums.len() {
            xor_all ^= (i as i32) ^ nums[i];
        }

        let shift = xor_all.trailing_zeros() as usize;

        let mut ans = vec![0, 0];
        for i in 0..nums.len() {
            if (i as i32) < n {
                ans[(i >> shift) & 1] ^= i as i32;
            }
            ans[(nums[i] >> shift) & 1] ^= nums[i];
        }
        ans
    }
}

fn main() {
    let mut num_size_str = String::new();
    io::stdin().read_line(&mut num_size_str).expect("Failed to read line");
    let mut num_size: i32 = num_size_str.trim().parse().expect("Invalid input");
    num_size += 2;

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let mut nums_mut = nums.clone();

    let solution = Solution {};
    let result = solution.get_sneaky_numbers(&mut nums_mut);

    for i in 0..2 {
        print!("{} ", result[i]);
    }
}