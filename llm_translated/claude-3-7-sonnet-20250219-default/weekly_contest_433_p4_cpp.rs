use std::io::{self, BufRead};
use std::collections::VecDeque;
use std::i32;

struct Solution;

impl Solution {
    pub fn min_max_subarray_sum(mut nums: Vec<i32>, k: i32) -> i64 {
        // Helper function to count the number of subarrays
        let count = |m: i32| -> i64 {
            if m > k {
                (m * 2 - k + 1) as i64 * k as i64 / 2
            } else {
                (m + 1) as i64 * m as i64 / 2
            }
        };

        // Function to calculate sum of minimum values of all subarrays
        let sum_subarray_mins = |nums: &Vec<i32>| -> i64 {
            let mut res: i64 = 0;
            let mut st: Vec<i32> = Vec::new();
            st.push(-1);
            
            for r in 0..nums.len() {
                while st.len() > 1 && nums[st[st.len() - 1] as usize] >= nums[r] {
                    let i = st.pop().unwrap();
                    let l = st[st.len() - 1];
                    let cnt = count(r as i32 - l - 1) - count(i - l - 1) - count(r as i32 - i - 1);
                    res += nums[i as usize] as i64 * cnt;
                }
                st.push(r as i32);
            }
            res
        };

        // Append a very small value to handle edge cases
        nums.push(i32::MIN / 2);
        let ans = sum_subarray_mins(&nums);
        
        // Negate all elements to calculate max subarrays (by finding min of negated values)
        for x in nums.iter_mut() {
            *x = -*x;
        }
        // Correct the last element (the sentinel value)
        *nums.last_mut().unwrap() *= -1;
        
        // Subtract the sum of minimum values of negated array
        ans - sum_subarray_mins(&nums)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse the first line containing n and k
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Parse the second line containing the array elements
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();
    
    let solution = Solution::min_max_subarray_sum(nums, k);
    println!("{}", solution);
    
    Ok(())
}