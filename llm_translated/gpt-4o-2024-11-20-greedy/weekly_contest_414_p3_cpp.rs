use std::io::{self, Write};
use std::cmp;

// Struct to encapsulate the solution logic
struct Solution;

impl Solution {
    // Function to find the maximum score
    pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut mx: i32 = 0;

        // Iterate through the array, updating the maximum and accumulating the score
        for i in 0..nums.len() - 1 {
            mx = cmp::max(mx, nums[i]);
            ans += mx as i64;
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums_size: usize = input.trim().parse().unwrap();

    let mut nums = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    nums = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // Ensure the input size matches the expected size
    assert_eq!(nums.len(), nums_size);

    // Create a solution instance and compute the result
    let sol = Solution;
    let result = sol.find_maximum_score(nums);

    // Print the result to stdout
    println!("{}", result);
}