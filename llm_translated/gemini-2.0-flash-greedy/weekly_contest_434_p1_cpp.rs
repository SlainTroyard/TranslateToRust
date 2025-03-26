use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn count_partitions(&self, nums: Vec<i32>) -> i32 {
        let s: i32 = nums.iter().sum();
        if s % 2 != 0 {
            0
        } else {
            (nums.len() as i32) - 1
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let sol = Solution {};
    println!("{}", sol.count_partitions(nums));
}