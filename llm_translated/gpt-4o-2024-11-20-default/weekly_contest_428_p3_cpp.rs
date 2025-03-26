use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn beautiful_splits(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // lcp[i][j] represents the Longest Common Prefix (LCP) between nums[i..] and nums[j..]
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse size");

    // Read the array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse element"))
        .collect();

    assert_eq!(nums.len(), n, "Mismatch between declared size and input array length");

    // Solve the problem
    let result = Solution::beautiful_splits(nums);

    // Output the result
    println!("{}", result);
}