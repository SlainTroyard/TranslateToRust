use std::io::{self, BufRead};
use std::collections::{VecDeque, VecDeque};

struct Solution;

impl Solution {
    fn count_non_decreasing_subarrays(nums: &Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut pos_r: Vec<usize> = vec![n; n];
        let mut st: Vec<usize> = Vec::new();

        for i in 0..n {
            let x = nums[i];
            while !st.is_empty() && x >= nums[*st.last().unwrap()] {
                pos_r[*st.last().unwrap()] = i;
                st.pop();
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and k
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // Read nums
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    // Calculate and print the result
    let solution = Solution;
    let result = solution.count_non_decreasing_subarrays(&nums, k);
    println!("{}", result);

    Ok(())
}