use std::collections::{VecDeque, HashMap};
use std::io::{self, BufRead};
use std::cmp::min;

struct Solution;

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut pos_r: Vec<usize> = vec![n; n];
        let mut st: Vec<usize> = Vec::new();

        // Process to determine the positions and adjacency structure
        for (i, &x) in nums.iter().enumerate() {
            while let Some(&top) = st.last() {
                if x >= nums[top] {
                    pos_r[top] = i;
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
        let mut q: VecDeque<usize> = VecDeque::new();

        // Two-pointer sliding window approach
        for (r, &x) in nums.iter().enumerate() {
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
                    cnt -= (out - nums[i]) * (min(pos_r[i], r + 1) - i) as i32;
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input: First line contains `n` and `k`
    let first_line = lines.next().unwrap().unwrap();
    let mut input_iter = first_line.split_whitespace();
    let n: usize = input_iter.next().unwrap().parse().unwrap();
    let k: i32 = input_iter.next().unwrap().parse().unwrap();

    // Read the `nums` array
    let nums_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line.split_whitespace()
                                  .map(|x| x.parse().unwrap())
                                  .collect();

    // Create Solution instance and perform calculation
    let solution = Solution;
    let result = solution.count_non_decreasing_subarrays(nums, k);

    // Output the result
    println!("{}", result);
}