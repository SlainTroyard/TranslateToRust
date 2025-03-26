use std::collections::VecDeque;
use std::io;
use std::cmp::min;

// This function counts non-decreasing subarrays with a specific constraint related to k
fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut pos_r: Vec<usize> = vec![n; n];
    let mut st: Vec<usize> = Vec::new();
    
    // Preprocessing to build the graph and position array
    for i in 0..n {
        let x = nums[i];
        while !st.is_empty() && x >= nums[*st.last().unwrap()] {
            let top = st.pop().unwrap();
            pos_r[top] = i;
        }
        if !st.is_empty() {
            g[*st.last().unwrap()].push(i);
        }
        st.push(i);
    }

    let mut ans: i64 = 0;
    let mut cnt = 0;
    let mut l = 0;
    let mut q: VecDeque<usize> = VecDeque::new();
    
    // Sliding window approach to count valid subarrays
    for r in 0..n {
        let x = nums[r];
        while !q.is_empty() && nums[*q.back().unwrap()] <= x {
            q.pop_back();
        }
        q.push_back(r);
        cnt += nums[*q.front().unwrap()] - x;
        
        // Adjust the window when the constraint is violated
        while cnt > k {
            let out = nums[l];
            for &i in &g[l] {
                if i > r {
                    break;
                }
                cnt -= (out - nums[i]) * (min(pos_r[i], r + 1) - i) as i32;
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

fn main() -> io::Result<()> {
    // Read n and k from the first line
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let parts: Vec<i32> = input.trim().split_whitespace()
                             .map(|s| s.parse().unwrap())
                             .collect();
    let (n, k) = (parts[0] as usize, parts[1]);
    
    // Read the nums array from the second line
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let nums: Vec<i32> = input.trim().split_whitespace()
                           .map(|s| s.parse().unwrap())
                           .collect();
    
    // Call the solution function and print the result
    println!("{}", count_non_decreasing_subarrays(&nums, k));
    
    Ok(())
}