use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn count_partitions(nums: &Vec<i32>) -> i32 {
        let s: i32 = nums.iter().sum();
        if s % 2 != 0 {
            0
        } else {
            (nums.len() - 1) as i32
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the vector elements
    let nums: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = Solution::count_partitions(&nums);
    println!("{}", result);

    Ok(())
}