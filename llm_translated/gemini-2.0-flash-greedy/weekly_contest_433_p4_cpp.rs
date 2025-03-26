use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn min_max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        // Helper function to calculate the count based on m and k
        let count = |m: i32| -> i64 {
            if m > k {
                let m_long = m as i64;
                let k_long = k as i64;
                (m_long * 2 - k_long + 1) * k_long / 2
            } else {
                let m_long = m as i64;
                (m_long + 1) * m_long / 2
            }
        };

        // Helper function to calculate the sum of subarray mins
        let sum_subarray_mins = |nums: &mut Vec<i32>| -> i64 {
            let mut res: i64 = 0;
            let mut st: Vec<i32> = Vec::new();
            st.push(-1);

            for r in 0..nums.len() {
                while st.len() > 1 && nums[st.last().unwrap() as usize] >= nums[r] {
                    let i = st.pop().unwrap();
                    let l = st.last().unwrap();

                    let cnt = count(r as i32 - l - 1) - count(i - l - 1) - count(r as i32 - i - 1);
                    res += nums[i as usize] as i64 * cnt;
                }
                st.push(r as i32);
            }
            res
        };

        let mut nums = nums;
        nums.push(i32::min_value() / 2); // Equivalent to INT_MIN / 2

        let mut ans = sum_subarray_mins(&mut nums);

        for x in &mut nums {
            *x = -*x;
        }
        *nums.last_mut().unwrap() *= -1;

        ans -= sum_subarray_mins(&mut nums);
        ans
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