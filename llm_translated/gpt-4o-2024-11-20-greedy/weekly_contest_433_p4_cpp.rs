use std::io::{self, BufRead};
use std::cmp;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn min_max_subarray_sum(nums: &mut Vec<i32>, k: i32) -> i64 {
        // Helper function to compute the sum of the first `m` natural numbers based on `k`
        let count = |m: i32| -> i64 {
            if m > k {
                (m as i64 * 2 - k as i64 + 1) * k as i64 / 2
            } else {
                (m as i64 + 1) * m as i64 / 2
            }
        };

        // Helper function to calculate the sum of subarray minimums
        let sum_subarray_mins = || -> i64 {
            let mut res = 0i64;
            let mut stack: Vec<i32> = Vec::new(); // stack of indices
            stack.push(-1); // initialize with -1 to represent the left boundary for first element

            for (r, &num) in nums.iter().enumerate() {
                let r = r as i32;

                while stack.len() > 1 && nums[*stack.last().unwrap() as usize] >= num {
                    let i = stack.pop().unwrap();
                    let l = *stack.last().unwrap();
                    let count_right = count(r - l - 1);
                    let count_left = count(i - l - 1);
                    let count_split = count(r - i - 1);
                    let cnt = count_right - count_left - count_split;
                    res += nums[i as usize] as i64 * cnt;
                }

                stack.push(r);
            }

            res
        };

        // Push sentinel value
        nums.push(i32::MIN / 2);

        // Calculate the result
        let mut ans = sum_subarray_mins();

        // Negate each element and recompute
        for x in nums.iter_mut() {
            *x = -*x;
        }
        nums.last_mut().map(|x| *x *= -1); // Restore sentinel value to positive state (current sentinel is `-i32::MIN`)

        ans -= sum_subarray_mins();

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line contains `n` and `k`
    let first_line = lines.next().unwrap().unwrap();
    let mut input = first_line.split_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let k: i32 = input.next().unwrap().parse().unwrap();

    // Next line contains the array `nums`
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure vector has the right size
    assert!(nums.len() == n);

    // Solve the problem
    let mut nums_mut = nums.clone(); // Make a mutable clone
    let solution = Solution::min_max_subarray_sum(&mut nums_mut, k);

    // Print the result
    println!("{}", solution);
}