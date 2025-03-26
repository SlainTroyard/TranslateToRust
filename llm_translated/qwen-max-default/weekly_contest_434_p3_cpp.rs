use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_frequency(nums: &Vec<i32>, k: i32) -> i32 {
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
    let mut lines = stdin.lock().lines();

    // Parse the first line for n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // Parse the second line for nums
    let nums_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    // Create an instance of Solution and call max_frequency
    let solution = Solution;
    let result = solution.max_frequency(&nums, k);

    // Output the result to stdout
    println!("{}", result);
}