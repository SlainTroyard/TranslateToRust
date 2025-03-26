use std::collections::{VecDeque, Vec};
use std::io;
use std::str::FromStr;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.trim().split_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let k = iter.next().unwrap().parse::<i32>().unwrap();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let nums: Vec<i32> = line
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let solution = Solution {};
    println!("{}", solution.count_non_decreasing_subarrays(nums, k));
}

struct Solution {}

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut pos_r: Vec<usize> = vec![n; n];
        let mut st: Vec<usize> = Vec::new();

        for i in 0..n {
            let x = nums[i];
            while !st.is_empty() && x >= nums[st[st.len() - 1]] {
                pos_r[st[st.len() - 1]] = i;
                st.pop();
            }
            if !st.is_empty() {
                g[st[st.len() - 1]].push(i);
            }
            st.push(i);
        }

        let mut ans: i64 = 0;
        let mut cnt: i32 = 0;
        let mut l: usize = 0;
        let mut q: VecDeque<usize> = VecDeque::new();

        for r in 0..n {
            let x = nums[r];
            while !q.is_empty() && nums[q[q.len() - 1]] <= x {
                q.pop_back();
            }
            q.push_back(r);
            cnt += nums[q[0]] - x;

            while cnt > k {
                let out = nums[l];
                for &i in &g[l] {
                    if i > r {
                        break;
                    }
                    cnt -= (out - nums[i]) * ((pos_r[i].min(r + 1) - i) as i32);
                }
                l += 1;
                if !q.is_empty() && q[0] < l {
                    q.pop_front();
                }
            }
            ans += (r - l + 1) as i64;
        }
        ans
    }
}