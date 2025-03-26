use std::cmp::min;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn minimum_sum_subarray(nums: Vec<i32>, l: usize, r: usize) -> i32 {
        let n = nums.len();
        let mut mini = i32::MAX;

        for i in 0..n {
            let mut currsum = 0;

            for j in i..n {
                currsum += nums[j];
                let length = j - i + 1;

                if length >= l && length <= r && currsum > 0 {
                    mini = min(mini, currsum);
                }
            }
        }

        if mini == i32::MAX {
            -1
        } else {
            mini
        }
    }
}

fn main() {
    // Read all input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Expected an integer for size of the array");

    // Read the array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Expected integers in the array"))
        .collect();

    assert_eq!(n, nums.len(), "Array size mismatch with input");

    // Read the range [l, r]
    let range: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Expected integers for range"))
        .collect();

    assert_eq!(range.len(), 2, "Expected exactly two integers for range");

    let (l, r) = (range[0], range[1]);

    // Compute the minimum sum subarray
    let solution = Solution;
    let result = solution.minimum_sum_subarray(nums, l, r);

    // Output the result
    println!("{}", result);
}