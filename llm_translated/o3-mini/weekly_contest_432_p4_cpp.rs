use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// Count the number of non-decreasing subarrays with the given property.
/// This function directly translates the logic in the given C++ code.
fn count_non_decreasing_subarrays(nums: &Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    
    // g: graph structure storing children for each index
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    // pos_r: for each index, its "right" position, default is n
    let mut pos_r: Vec<usize> = vec![n; n];
    
    // Use a vector as a stack to simulate the C++ stack.
    let mut st: Vec<usize> = Vec::new();
    
    // Build the tree structure and compute pos_r
    for i in 0..n {
        let x = nums[i];
        while let Some(&last_idx) = st.last() {
            if x >= nums[last_idx] {
                pos_r[last_idx] = i;
                st.pop();
            } else {
                break;
            }
        }
        // If the stack is not empty, then the top of the stack is the parent of i.
        if let Some(&top) = st.last() {
            g[top].push(i);
        }
        st.push(i);
    }
    
    // Initialize variables to compute the answer.
    let mut ans: i64 = 0;
    let mut cnt: i64 = 0;
    let mut l: usize = 0;
    // q is a deque to maintain a monotonic decreasing sequence of indices.
    let mut q: VecDeque<usize> = VecDeque::new();
    
    for r in 0..n {
        let x = nums[r];
        // Maintain the deque so that the value at q.back() are > x.
        while let Some(&back) = q.back() {
            if nums[back] <= x {
                q.pop_back();
            } else {
                break;
            }
        }
        q.push_back(r);
        // The front of q holds the index with the maximum value in the current window.
        cnt += (nums[*q.front().unwrap()] - x) as i64;
        
        // Adjust the left pointer until cnt <= k.
        while cnt > k as i64 {
            let out = nums[l];
            // For each child index in g[l] adjust cnt.
            for &i in &g[l] {
                if i > r {
                    break;
                }
                // Calculate the effective interval via min(pos_r[i], r + 1)
                let min_index = if pos_r[i] < r + 1 { pos_r[i] } else { r + 1 };
                cnt -= ((out - nums[i]) as i64) * ((min_index - i) as i64);
            }
            l += 1;
            // Remove indices from the deque that are no longer in the window.
            if let Some(&front) = q.front() {
                if front < l {
                    q.pop_front();
                }
            }
        }
        // Add the number of valid subarrays ending at r.
        ans += (r - l + 1) as i64;
    }
    
    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    // Use stdin lock to read input lines
    let stdin = io::stdin();
    let mut input = String::new();
    
    // Read complete input
    for line in stdin.lock().lines() {
        let line = line?;
        input.push_str(&line);
        input.push(' ');
    }
    
    // Tokenize the input while preserving spacing as in the original C++ I/O.
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut iter = tokens.into_iter();
    
    // The first value is n (number of elements) and the second value is k.
    let n: usize = iter
        .next()
        .ok_or("Missing n")?
        .parse()
        .map_err(|_| "Failed parsing n")?;
    let k: i32 = iter
        .next()
        .ok_or("Missing k")?
        .parse()
        .map_err(|_| "Failed parsing k")?;
    
    // Read the next n numbers.
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = iter
            .next()
            .ok_or("Missing a number in array")?
            .parse()
            .map_err(|_| "Failed parsing a number in the array")?;
        nums.push(num);
    }
    
    // Compute and print the answer.
    let result = count_non_decreasing_subarrays(&nums, k);
    println!("{}", result);
    
    // Ensure stdout is flushed.
    io::stdout().flush()?;
    Ok(())
}