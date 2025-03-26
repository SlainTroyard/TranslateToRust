use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn min_max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums; // Make nums mutable to allow push and modification

        // Helper function to count the number of subarrays of size m
        let count = |m: i32, k: i32| -> i64 {
            if m > k {
                ((m as i64 * 2 - k as i64 + 1) as i64 * k as i64) / 2
            } else {
                ((m as i64 + 1) as i64 * m as i64) / 2
            }
        };

        // Helper function to calculate the sum of subarray mins
        let sum_subarray_mins = |nums: &mut Vec<i32>, k: i32| -> i64 {
            let mut res: i64 = 0;
            let mut st: Vec<i32> = Vec::new();
            st.push(-1);

            for r in 0..nums.len() {
                while st.len() > 1 && nums[st.last().unwrap() as usize] >= nums[r] {
                    let i = st.pop().unwrap();
                    let l = st.last().unwrap();

                    let cnt = count(r as i32 - l - 1, k) - count(i - l - 1, k) - count(r as i32 - i - 1, k);
                    res += nums[i as usize] as i64 * cnt;
                }
                st.push(r as i32);
            }

            res
        };

        // Main logic:

        nums.push(i32::min_value() / 2); // sentinel value

        let ans1 = sum_subarray_mins(&mut nums, k);

        for x in nums.iter_mut() {
            *x = -*x;
        }
        let last_index = nums.len() - 1;
        nums[last_index] *= -1;

        let ans2 = sum_subarray_mins(&mut nums, k);

        ans1 - ans2
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();

    let n: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let sol = Solution {};
    println!("{}", sol.min_max_subarray_sum(nums, k));
}