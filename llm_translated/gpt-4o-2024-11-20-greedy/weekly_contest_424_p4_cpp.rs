use std::cmp::{max, min};
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min_l = i32::MAX;
        let mut max_r = 0;

        // Find min_l and max_r based on the conditions
        for i in 0..n {
            if nums[i] != -1
                && ((i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1))
            {
                min_l = min(min_l, nums[i]);
                max_r = max(max_r, nums[i]);
            }
        }

        let mut ans = 0;

        // Closure to update the answer
        let mut update_ans = |l: i32, r: i32, big: bool| {
            let mut d = (min(r - min_l, max_r - l) + 1) / 2;
            if big {
                d = min(d, (max_r - min_l + 2) / 3);
            }
            ans = max(ans, d);
        };

        let mut pre_i = -1;

        // Iterate through the array to calculate the result
        for i in 0..n {
            if nums[i] == -1 {
                continue;
            }
            if pre_i >= 0 {
                if i - pre_i == 1 {
                    ans = max(ans, (nums[i] - nums[pre_i]).abs());
                } else {
                    update_ans(
                        min(nums[pre_i], nums[i]),
                        max(nums[pre_i], nums[i]),
                        i - pre_i > 2,
                    );
                }
            } else if i > 0 {
                update_ans(nums[i], nums[i], false);
            }
            pre_i = i as i32;
        }

        if pre_i >= 0 && pre_i < (n as i32 - 1) {
            update_ans(nums[pre_i as usize], nums[pre_i as usize], false);
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse n");

    // Read the array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse number"))
        .collect();

    // Ensure the input size matches the expected size
    assert_eq!(nums.len(), n, "Input size does not match the specified n");

    // Solve the problem
    let result = Solution::min_difference(nums);

    // Print the result
    println!("{}", result);
}