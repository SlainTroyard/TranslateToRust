// Problem: Weekly Contest 429 Problem 1

use std::io;
use std::io::Read;

fn minimum_operations(nums: &[i32]) -> i32 {
    let mut count = [0; 101];
    for i in (0..nums.len()).rev() {
        count[nums[i] as usize] += 1;
        if count[nums[i] as usize] > 1 {
            return ((i as i32) + 3) / 3;
        }
    }
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse()?;

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = minimum_operations(&nums);
    println!("{}", result);

    Ok(())
}