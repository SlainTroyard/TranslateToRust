// Problem: Weekly Contest 434 Problem 1
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn count_partitions(nums: &Vec<i32>) -> i32 {
        // Calculate the sum of all elements in the vector
        let s: i32 = nums.iter().sum();
        
        // If the sum is odd, we can't partition it into two equal parts
        if s % 2 != 0 {
            0
        } else {
            // If sum is even, return n-1 as per the problem's logic
            nums.len() as i32 - 1
        }
    }
}

fn main() {
    // Set up buffered reader for stdin
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    
    // Read the number of elements
    let n: usize = input.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the array elements
    let nums: Vec<i32> = input
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Create solution instance and solve
    let sol = Solution;
    println!("{}", Solution::count_partitions(&nums));
}