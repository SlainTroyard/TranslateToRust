use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn count_partitions(nums: &[i32]) -> i32 {
        let s: i32 = nums.iter().sum();
        if s % 2 != 0 {
            0
        } else {
            nums.len() as i32 - 1
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = Solution::count_partitions(&nums);
    println!("{}", result);

    Ok(())
}