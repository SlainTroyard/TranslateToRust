use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_frequency(nums: &Vec<i32>, k: i32) -> i32 {
        let mut f0 = 0;
        let mut f1 = [0; 51]; // Array to track frequencies, assuming max value is 50
        let mut max_f1 = 0;
        let mut f2 = 0;
        
        for &x in nums {
            // Update f2: max frequency considering the current element if it equals k
            f2 = max(f2, max_f1) + if x == k { 1 } else { 0 };
            
            // Update f1: frequency for the current number
            f1[x as usize] = max(f1[x as usize], f0) + 1;
            
            // Update f0: count of elements equal to k
            f0 += if x == k { 1 } else { 0 };
            
            // Update max_f1
            max_f1 = max(max_f1, f1[x as usize]);
        }
        
        max(max_f1, f2)
    }
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Parse nums
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print the result
    let solution = Solution;
    println!("{}", Solution::max_frequency(&nums, k));
}