use std::collections::{VecDeque, VecDequeIter};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and k from the first line
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read the array nums from the second line
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let solution = Solution;
    println!("{}", solution.count_non_decreasing_subarrays(nums, k));
}

struct Solution;

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut g: Vec<Vec<usize>> = vec![vec![]; n];
        let mut pos_r: Vec<usize> = vec![n; n];
        let mut st: Vec<usize> = Vec::new();
        
        // Build the graph g and pos_r
        for i in 0..n {
            let x = nums[i];
            while !st.is_empty() && x >= nums[*st.last().unwrap()] {
                pos_r[st.pop().unwrap()] = i;
            }
            if !st.is_empty() {
                g[*st.last().unwrap()].push(i);
            }
            st.push(i);
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