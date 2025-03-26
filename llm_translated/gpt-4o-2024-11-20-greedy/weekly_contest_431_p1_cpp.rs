use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut m = 0;

        // Find the maximum value in the array
        for &x in &nums {
            m = max(m, x);
        }

        // Create a vector of factors for each number up to m
        let mut fac = vec![Vec::new(); (m + 1) as usize];
        for i in 2..=m {
            if fac[i as usize].is_empty() {
                let mut j = i;
                while j <= m {
                    fac[j as usize].push(i);
                    j += i;
                }
            }
        }

        let mut ans = 2;
        let mut vis = vec![false; (m + 1) as usize];
        let mut j = 0;

        for i in 0..n {
            while j < n {
                // Closure to check if the current number can be added
                let check = || {
                    for &p in &fac[nums[j] as usize] {
                        if vis[p as usize] {
                            return false;
                        }
                    }
                    true
                };

                if check() {
                    for &p in &fac[nums[j] as usize] {
                        vis[p as usize] = true;
                    }
                    j += 1;
                } else {
                    break;
                }
            }

            ans = max(ans, (j - i) as i32);

            // Remove the factors of nums[i] from the visited array
            for &p in &fac[nums[i] as usize] {
                vis[p as usize] = false;
            }
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let num_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse numSize");

    // Read the array of numbers
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse number"))
        .collect();

    assert_eq!(nums.len(), num_size, "Input size does not match numSize");

    // Solve the problem
    let solution = Solution;
    let result = solution.max_length(nums);

    // Print the result
    println!("{}", result);
}