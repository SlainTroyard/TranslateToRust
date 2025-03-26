// Problem: Weekly Contest 432 Problem 4
use std::io::{self, BufRead};
use std::collections::{VecDeque, HashMap};

struct Solution;

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut g = vec![Vec::new(); n];
        let mut pos_r = vec![n as i32; n];
        let mut st = Vec::new();

        // Building g and pos_r using a stack
        for i in 0..n {
            let x = nums[i];
            while let Some(&top) = st.last() {
                if x >= nums[top] {
                    pos_r[top] = i as i32;
                    st.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = st.last() {
                g[top].push(i);
            }
            st.push(i);
        }

        let mut ans = 0i64;
        let mut cnt = 0;
        let mut l = 0;
        let mut q = VecDeque::new();

        for r in 0..n {
            let x = nums[r];
            while let Some(&back) = q.back() {
                if nums[back] <= x {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back(r);
            cnt += nums[q.front().unwrap()] - x;

            while cnt > k {
                let out = nums[l];
                for &i in &g[l] {
                    if i > r {
                        break;
                    }
                    cnt -= (out - nums[i]) * ((pos_r[i] - i as i32).min((r + 1) as i32) - i as i32);
                }
                l += 1;
                if let Some(&front) = q.front() {
                    if front < l {
                        q.pop_front();
                    }
                }
            }
            ans += (r - l + 1) as i64;
        }

        ans
    }
}

fn main() {
    // Reading input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse `n` and `k` from the first input line
    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let n: usize = first_iter.next().unwrap().parse().unwrap();
    let k: i32 = first_iter.next().unwrap().parse().unwrap();

    // Parse array `nums` from the second input line
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Solve the problem and output the result
    let solution = Solution;
    let result = solution.count_non_decreasing_subarrays(nums, k);
    println!("{}", result);
}