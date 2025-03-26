use std::cmp;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: usize, r: usize) -> i32 {
        let n = nums.len();
        let mut mini = i32::MAX;

        for i in 0..n {
            let mut currsum = 0;
            for j in i..n {
                currsum += nums[j];
                let length = j - i + 1;

                if length >= l && length <= r && currsum > 0 {
                    mini = cmp::min(mini, currsum);
                }
            }
        }

        if mini == i32::MAX {
            -1 // Return -1 if no valid subarray is found
        } else {
            mini
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Input the array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Input the range [l, r]
    let range: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let l = range[0];
    let r = range[1];

    // Compute the minimum sum subarray
    let solution = Solution;
    let result = solution.minimum_sum_subarray(nums, l, r);

    // Output the result
    println!("{}", result);
}