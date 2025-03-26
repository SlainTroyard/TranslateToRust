use std::io::{self, BufRead};
use std::collections::VecDeque;

// Simple Stack implementation for integers
struct Stack {
    data: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push(&mut self, item: i32) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    fn top(&self) -> Option<i32> {
        self.data.last().copied()
    }
}

// Calculate the number of non-decreasing subarrays with sum at least k
fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let nums_size = nums.len();
    
    // Create graph g and pos_r array
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); nums_size];
    let mut pos_r: Vec<usize> = vec![nums_size; nums_size];
    
    // Use stack to find the next greater or equal element for each position
    let mut st = Stack::new();
    for i in 0..nums_size {
        let x = nums[i];
        while let Some(top) = st.top() {
            if x >= nums[top as usize] {
                pos_r[top as usize] = i;
                st.pop();
            } else {
                break;
            }
        }
        if let Some(top) = st.top() {
            g[top as usize].push(i);
        }
        st.push(i as i32);
    }
    
    // Calculate result
    let mut ans: i64 = 0;
    let mut cnt = 0;
    let mut l = 0;
    let mut q = VecDeque::new();
    
    for r in 0..nums_size {
        let x = nums[r];
        while let Some(&back) = q.back() {
            if nums[back] <= x {
                q.pop_back();
            } else {
                break;
            }
        }
        q.push_back(r);
        cnt += nums[*q.front().unwrap()] - x;
        
        while cnt > k {
            let out = nums[l];
            for &i in &g[l] {
                if i > r {
                    break;
                }
                let min_pos = std::cmp::min(pos_r[i], r + 1);
                cnt -= (out - nums[i]) * ((min_pos - i) as i32);
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let nums_size: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .take(nums_size)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Call function to calculate result
    let result = count_non_decreasing_subarrays(&nums, k);
    
    // Output result
    println!("{}", result);
    
    Ok(())
}