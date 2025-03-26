use std::io;
use std::vec::Vec;

struct Solution {}

impl Solution {
    pub fn get_sneaky_numbers(&self, nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32 - 2;
        let mut xor_all = n ^ (n + 1);
        for i in 0..nums.len() {
            xor_all ^= (i as i32) ^ nums[i];
        }
        let shift = xor_all.trailing_zeros() as usize;

        let mut ans = vec![0; 2];
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
    io::stdin().read_line(&mut num_size_str).unwrap();
    let mut num_size: i32 = num_size_str.trim().parse().unwrap();
    num_size += 2;

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).unwrap();
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let solution = Solution {};
    let result = solution.get_sneaky_numbers(nums);

    for i in 0..2 {
        print!("{} ", result[i]);
    }
    println!();
}