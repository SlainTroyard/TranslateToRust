use std::collections::{VecDeque, LinkedList};
use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line to get n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    // Parse the second line to get the nums vector
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Create an instance of Solution and call the function
    let solution = Solution;
    let result = solution.count_non_decreasing_subarrays(nums, k);

    // Print the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    pub fn count_non_decreasing_subarrays(&self, nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut pos_r: Vec<usize> = vec![n; n];
        let mut st: LinkedList<usize> = LinkedList::new();

        // Build the graph and pos_r array
        for i in 0..n {
            let x = nums[i];
            while !st.is_empty() && x >= nums[*st.back().unwrap()] {
                pos_r[*st.back().unwrap()] = i;
                st.pop_back();
            }
            if !st.is_empty() {
                g[*st.back().unwrap()].push(i);
            }
            st.push_back(i);
        }

        let mut ans: i64 = 0;
        let mut cnt: i32 = 0;
        let mut l: usize = 0;
        let mut q: VecDeque<usize> = VecDeque::new();

        for r in 0..n {
            let x = nums[r];
            while !q.is_empty() && nums[*q.back().unwrap()] <= x {
                q.pop_back();
            }
            q.push_back(r);
            cnt += nums[*q.front().unwrap()] - x;

            while cnt > k {
                let out = nums[l];
                for &i in &g[l] {
                    if i > r {
                        break;
                    }
                    cnt -= (out - nums[i]) * (pos_r[i].min(r + 1) - i) as i32;
                }
                l += 1;
                if !q.is_empty() && *q.front().unwrap() < l {
                    q.pop_front();
                }
            }
            ans += (r - l + 1) as i64;
        }
        ans
    }
}