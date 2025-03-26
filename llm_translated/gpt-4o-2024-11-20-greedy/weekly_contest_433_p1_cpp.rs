use std::io::{self, BufRead};
use std::cmp;

// Helper function to calculate prefix sums
fn partial_sum(nums: &[i32]) -> Vec<i32> {
    let mut s = vec![0; nums.len() + 1];
    for (i, &num) in nums.iter().enumerate() {
        s[i + 1] = s[i] + num;
    }
    s
}

struct Solution;

impl Solution {
    // Function to calculate the sum based on the given algorithm
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let s = partial_sum(&nums);

        let mut ans = 0;
        for i in 0..n {
            // Calculating the range sum using the difference of prefix sums
            let start_index = cmp::max(i as isize - nums[i] as isize, 0) as usize;
            ans += s[i + 1] - s[start_index];
        }
        ans
    }
}

fn main() {
    // Read from standard input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line: number of elements (n)
    let n: usize = lines
        .next()
        .expect("Expected input")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse n");

    // Read the second line: space-separated numbers
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse number"))
        .collect();

    // Ensure the input matches the expected size
    assert_eq!(nums.len(), n, "Input array length does not match n");

    // Call the solution's function and print the result
    let solution = Solution;
    let result = solution.subarray_sum(nums);
    println!("{}", result);
}