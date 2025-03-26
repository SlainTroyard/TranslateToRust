use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn beautiful_splits(nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        // lcp[i][j] represents the Longest Common Prefix (LCP) between s[i:] and s[j:]
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
                if i as i32 <= j as i32 - i as i32 && lcp[0][i] >= i as i32 || lcp[i][j] >= j as i32 - i as i32 {
                    ans += 1;
                }
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input: size of array and array elements
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    let nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = Solution::beautiful_splits(&nums);
    println!("{}", result);

    Ok(())
}