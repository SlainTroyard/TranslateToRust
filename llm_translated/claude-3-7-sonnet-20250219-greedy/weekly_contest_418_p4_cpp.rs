use std::io::{self, BufRead};
use std::iter::Iterator;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let mx = *nums.iter().max().unwrap();
        let mut cnt_x = vec![0; mx as usize + 1];
        for x in nums {
            cnt_x[x as usize] += 1;
        }

        let mut cnt_gcd = vec![0i64; mx as usize + 1];
        for i in (1..=mx as usize).rev() {
            let mut c = 0;
            for j in (i..=mx as usize).step_by(i) {
                c += cnt_x[j];
                cnt_gcd[i] -= cnt_gcd[j];
            }
            cnt_gcd[i] += (c as i64) * (c - 1) as i64 / 2;
        }
        
        // Calculate partial sum (equivalent to std::partial_sum in C++)
        for i in 1..=mx as usize {
            cnt_gcd[i] += cnt_gcd[i-1];
        }

        let mut ans = vec![0; queries.len()];
        for i in 0..queries.len() {
            // Find upper bound (equivalent to std::upper_bound in C++)
            ans[i] = cnt_gcd.iter()
                .position(|&x| x > queries[i])
                .unwrap_or(cnt_gcd.len()) as i32;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read nums
    let nums: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read q
    let q: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read queries
    let queries: Vec<i64> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Solve the problem
    let ans = Solution::gcd_values(nums, queries);
    
    // Print the result
    for (i, x) in ans.iter().enumerate() {
        print!("{}", x);
        if i < ans.len() - 1 {
            print!(" ");
        }
    }
    println!();
    
    Ok(())
}