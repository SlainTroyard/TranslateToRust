use std::io::{self, BufRead, Write};

struct Solution;

impl Solution {
    pub fn max_frequency(nums: &Vec<i32>, k: i32) -> i32 {
        let mut f0 = 0;
        let mut f1 = [0; 51];
        let mut max_f1 = 0;
        let mut f2 = 0;

        for &x in nums.iter() {
            f2 = f2.max(max_f1) + if x == k { 1 } else { 0 };
            f1[x as usize] = f1[x as usize].max(f0) + 1;
            f0 += if x == k { 1 } else { 0 };
            max_f1 = max_f1.max(f1[x as usize]);
        }

        max_f1.max(f2)
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();
    stdin_lock.read_line(&mut buffer).expect("Failed to read line");
    let input: Vec<i32> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();
    let n = input[0] as usize;
    let k = input[1];

    buffer.clear();
    let mut nums = vec![0; n];
    for i in 0..n {
        stdin_lock.read_line(&mut buffer).expect("Failed to read line");
        nums[i] = buffer.trim().parse().expect("Failed to parse integer");
        buffer.clear();
    }

    // Solve the problem
    let solution = Solution;
    let result = solution.max_frequency(&nums, k);

    // Write output to stdout
    println!("{}", result);
}