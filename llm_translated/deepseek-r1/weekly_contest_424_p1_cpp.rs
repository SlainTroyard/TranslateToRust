use std::io::{self, Read};

struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;

        // Compute left prefix sums (sum of elements before index i)
        let mut left = vec![0; n];
        for i in 1..n {
            left[i] = left[i - 1] + nums[i - 1];
        }

        // Compute right prefix sums (sum of elements after index i)
        let mut right = vec![0; n];
        for j in (0..n - 1).rev() {
            right[j] = right[j + 1] + nums[j + 1];
        }

        // Check each element for valid selection conditions
        for i in 0..n {
            if nums[i] != 0 {
                continue;
            }
            let l = left[i];
            let r = right[i];
            if l == r {
                res += 2;
            } else if (l - r).abs() == 1 {
                res += 1;
            }
        }

        res
    }
}

fn main() {
    // Read all input as a string and split into tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    // Parse the number of elements
    let n: usize = tokens.next().unwrap().parse().unwrap();

    // Parse the nums vector
    let nums: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();

    // Compute and print the result
    let result = Solution::count_valid_selections(nums);
    println!("{}", result);
}