use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn min_max_subarray_sum(mut nums: Vec<i32>, k: i32) -> i64 {
        // Helper function to calculate count based on m and k
        let count = |m: i32| -> i64 {
            if m > k {
                (m * 2 - k + 1) as i64 * k as i64 / 2
            } else {
                (m + 1) as i64 * m as i64 / 2
            }
        };

        // Function to calculate sum of subarray minimums
        let sum_subarray_mins = || -> i64 {
            let mut res: i64 = 0;
            let mut st: Vec<i32> = vec![-1];
            for r in 0..nums.len() {
                while st.len() > 1 && nums[*st.last().unwrap() as usize] >= nums[r] {
                    let i = st.pop().unwrap();
                    let l = *st.last().unwrap();
                    let cnt = count(r as i32 - l - 1) - count(i - l - 1) - count(r as i32 - i - 1);
                    res += nums[i as usize] as i64 * cnt;
                }
                st.push(r as i32);
            }
            res
        };

        // Add sentinel value at the end (equivalent to INT_MIN / 2)
        nums.push(i32::MIN / 2);
        
        // Calculate sum of subarray minimums
        let ans = sum_subarray_mins();
        
        // Negate all elements
        for x in &mut nums {
            *x = -*x;
        }
        
        // Fix the sentinel value
        *nums.last_mut().unwrap() *= -1;
        
        // Return the difference
        ans - sum_subarray_mins()
    }
}

fn main() {
    // Parse input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and k from the first line
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = parts[0];
    let k = parts[1];
    
    // Read the array elements from the second line
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .take(n as usize)
        .collect();
    
    // Solve and print the result
    let sol = Solution;
    println!("{}", Solution::min_max_subarray_sum(nums, k));
}