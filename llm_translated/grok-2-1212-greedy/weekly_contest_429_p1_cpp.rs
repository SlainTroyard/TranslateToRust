use std::collections::HashSet;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn minimum_operations(nums: &Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        for (i, &num) in nums.iter().rev().enumerate() {
            if !seen.insert(num) {
                // num already in seen
                return (nums.len() - i - 1) as i32 / 3 + 1;
            }
        }
        0
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input size of the array
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Input elements of the array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .take(n)
        .collect();

    // Call the function and output the result
    let result = Solution::minimum_operations(&nums);
    println!("{}", result);

    Ok(())
}