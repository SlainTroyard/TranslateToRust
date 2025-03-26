use std::cmp::{max, min};
use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut curr = 1;
        let mut ans = 0;

        // Traverse through the nums array
        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                curr += 1; // Increase the length of the current increasing subarray
            } else {
                prev = curr; // Update the previous subarray length
                curr = 1; // Reset current subarray length
            }
            // Update the answer by considering both the current and previous subarray lengths
            ans = max(ans, max(curr / 2, min(prev, curr)));
        }
        ans
    }
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

    let sol = Solution {};
    let result = sol.max_increasing_subarrays(nums);

    println!("{}", result);

    Ok(())
}