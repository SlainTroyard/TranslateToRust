// Problem: Weekly Contest 428 Problem 3
use std::io::{self, Write};

struct Solution;

impl Solution {
    pub fn beautiful_splits(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // `lcp[i][j]` represents the Longest Common Prefix (LCP) between nums[i..] and nums[j..]
        let mut lcp = vec![vec![0; n + 1]; n + 1];
        for i in (0..n).rev() {
            for j in (i..n).rev() {
                if nums[i] == nums[j] {
                    lcp[i][j] = lcp[i + 1][j + 1] + 1;
                }
            }
        }

        let mut ans = 0;
        for i in 1..n - 1 {
            for j in i + 1..n {
                // Check if the split satisfies the beautiful condition
                if i <= j - i && lcp[0][i] >= i || lcp[i][j] >= j - i {
                    ans += 1;
                }
            }
        }

        ans
    }
}

fn main() {
    let mut input = String::new();

    // Read the size of the array
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    // Read the array elements
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    // Check if the input array size matches the size provided
    assert_eq!(
        nums.len(),
        n,
        "The number of elements in the array does not match `n`"
    );

    // Calculate and print the result
    let result = Solution::beautiful_splits(nums);
    println!("{}", result);
}