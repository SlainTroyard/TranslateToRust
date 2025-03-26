// Problem: Weekly Contest 428 Problem 3
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn beautiful_splits(nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        // lcp[i][j] represents the Longest Common Prefix (LCP) between s[i:] and s[j:]
        let mut lcp = vec![vec![0; n + 1]; n + 1];
        
        for i in (0..n).rev() {
            for j in (i..n).rev() {
                if nums[i] == nums[j] {
                    lcp[i][j] = lcp[i + 1][j + 1] + 1;
                }
            }
        }

        let mut ans = 0;
        for i in 1..n - 1 {
            for j in i + 1..n {
                // Check if the split satisfies the beautiful condition
                if i <= j - i && lcp[0][i] >= i || lcp[i][j] >= j - i {
                    ans += 1;
                }
            }
        }
        ans
    }
}

fn main() {
    // Set up the stdin reader
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input: size of array
    let n: usize = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .trim()
        .parse()
        .expect("Failed to parse n");
    
    // Input: array elements
    let nums: Vec<i32> = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse number"))
        .collect();
    
    // Calculate and print the result
    let result = Solution::beautiful_splits(&nums);
    println!("{}", result);
}