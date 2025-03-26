use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    // Function to calculate the maximum score
    fn find_maximum_score(nums: Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut mx: i32 = 0;
        for i in 0..nums.len() - 1 {
            mx = mx.max(nums[i]); // Update maximum value so far
            ans += mx as i64;    // Accumulate the score
        }
        ans
    }
}

fn main() {
    // Read the input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line contains the size of the nums vector
    let nums_size: usize = lines
        .next()
        .expect("Expected input for numsSize")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse numsSize");

    // Second line contains the space-separated nums array
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input for nums array")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse element in nums"))
        .collect();

    // Ensure the input size matches the specified length
    assert_eq!(nums.len(), nums_size, "Input size mismatch");

    // Initialize the solution instance and compute the result
    let sol = Solution {};
    let result = sol.find_maximum_score(nums);

    // Print the result to stdout
    println!("{}", result);
}