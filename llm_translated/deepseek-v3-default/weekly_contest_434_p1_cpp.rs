use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn count_partitions(nums: Vec<i32>) -> i32 {
        let s: i32 = nums.iter().sum();
        if s % 2 != 0 {
            0
        } else {
            nums.len() as i32 - 1
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line which contains the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the second line which contains the list of numbers
    let nums_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Ensure we have exactly `n` numbers
    assert_eq!(nums.len(), n);
    
    let sol = Solution;
    println!("{}", sol.count_partitions(nums));
}