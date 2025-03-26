use std::collections::VecDeque;
use std::io::{self, BufRead};

fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let nums_size = nums.len();
    
    // Create graph g and pos_r array
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); nums_size];
    let mut pos_r: Vec<usize> = vec![nums_size; nums_size];
    
    // Using stack to find the next greater or equal element for each position
    let mut st: Vec<usize> = Vec::new();
    for i in 0..nums_size {
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
    
    // Calculate result
    let mut ans: i64 = 0;
    let mut cnt = 0;
    let mut l = 0;
    let mut q: VecDeque<usize> = VecDeque::new();
    
    for r in 0..nums_size {
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
                let min_pos = if pos_r[i] < r + 1 { pos_r[i] } else { r + 1 };
                cnt -= (out - nums[i]) * ((min_pos - i) as i32);
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

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read numsSize and k
    let first_line = match lines.next() {
        Some(line) => match line {
            Ok(l) => l,
            Err(_) => {
                eprintln!("Error reading input for numsSize and k");
                return;
            }
        },
        None => {
            eprintln!("Error reading input for numsSize and k");
            return;
        }
    };
    
    let mut iter = first_line.split_whitespace();
    let nums_size: usize = match iter.next().and_then(|s| s.parse().ok()) {
        Some(n) => n,
        None => {
            eprintln!("Error reading input for numsSize");
            return;
        }
    };
    
    let k: i32 = match iter.next().and_then(|s| s.parse().ok()) {
        Some(n) => n,
        None => {
            eprintln!("Error reading input for k");
            return;
        }
    };
    
    // Read the array
    let mut nums = Vec::with_capacity(nums_size);
    for i in 0..nums_size {
        let num_line = match lines.next() {
            Some(line) => match line {
                Ok(l) => l,
                Err(_) => {
                    eprintln!("Error reading input for nums[{}]", i);
                    return;
                }
            },
            None => {
                eprintln!("Error reading input for nums[{}]", i);
                return;
            }
        };
        
        let num: i32 = match num_line.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Error reading input for nums[{}]", i);
                return;
            }
        };
        
        nums.push(num);
    }
    
    // Call function to calculate result
    let result = count_non_decreasing_subarrays(&nums, k);
    
    // Output result
    println!("{}", result);
}