use std::io::{self, BufRead, Write};

/// Represents the solution to the problem.
struct Solution;

impl Solution {
    /// Calculates the number of beautiful splits in the given array.
    fn beautiful_splits(nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        // lcp[i][j] represents the Longest Common Prefix (LCP) between s[i..] and s[j..]
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
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    // Read the size of the array
    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Invalid input");

    // Read the array elements
    input.clear();
    stdin_lock.read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    // Calculate and print the result
    let result = Solution::beautiful_splits(&nums);
    writeln!(stdout_lock, "{}", result).unwrap();
}