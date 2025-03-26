use std::io::{self, BufRead};

/// Computes the sum of subarrays based on the given algorithm.
fn subarray_sum(nums: &Vec<i32>) -> i32 {
    let n = nums.len();
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + nums[i];
    }

    let mut ans = 0;
    for i in 0..n {
        ans += s[i + 1] - s[(i as i32 - nums[i]).max(0) as usize];
    }
    ans
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line to get the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse n");

    // Parse the second line to get the array of integers
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse integer"))
        .collect();

    // Create an instance of the Solution and compute the result
    let solution = subarray_sum(&nums);

    // Output the result to stdout
    println!("{}", solution);
}