use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and k from the first line
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // Read the array of numbers from the second line
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Create a Solution instance and compute the result
    let sol = Solution;
    let result = sol.min_max_subarray_sum(nums, k);

    // Print the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    pub fn min_max_subarray_sum(&self, nums: Vec<i32>, k: i32) -> i64 {
        let count = |m: i32| -> i64 {
            if m > k {
                (m * 2 - k + 1) as i64 * k as i64 / 2
            } else {
                (m + 1) as i64 * m as i64 / 2
            }
        };

        let sum_subarray_mins = |nums: &Vec<i32>| -> i64 {
            let mut res = 0;
            let mut st = VecDeque::new();
            st.push_back(-1);
            for r in 0..nums.len() {
                while st.len() > 1 && nums[*st.back().unwrap() as usize] >= nums[r] {
                    let i = st.pop_back().unwrap();
                    let l = *st.back().unwrap();
                    let cnt = count((r as i32) - l - 1) - count(i - l - 1) - count((r as i32) - i - 1);
                    res += nums[i as usize] as i64 * cnt;
                }
                st.push_back(r as i32);
            }
            res
        };

        let mut nums = nums;
        nums.push(i32::MIN / 2);
        let mut ans = sum_subarray_mins(&nums);
        for x in &mut nums {
            *x = -*x;
        }
        *nums.last_mut().unwrap() *= -1;
        ans -= sum_subarray_mins(&nums);
        ans
    }
}