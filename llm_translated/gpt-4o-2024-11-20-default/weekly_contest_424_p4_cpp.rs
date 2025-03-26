use std::io::{self, BufRead};
use std::cmp::{min, max};

struct Solution;

impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min_l = i32::MAX;
        let mut max_r = 0;

        // Find the minimum and maximum of relevant numbers adjacent to -1
        for i in 0..n {
            if nums[i] != -1
                && ((i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1))
            {
                min_l = min(min_l, nums[i]);
                max_r = max(max_r, nums[i]);
            }
        }

        let mut ans = 0;

        // Closure to handle intermediate result updates
        let mut update_ans = |l: i32, r: i32, big: bool| {
            let mut d = (min(r - min_l, max_r - l) + 1) / 2;
            if big {
                d = min(d, (max_r - min_l + 2) / 3);
            }
            ans = max(ans, d);
        };

        let mut pre_i: Option<usize> = None;

        // Main logic of traversing nums
        for i in 0..n {
            if nums[i] == -1 {
                continue;
            }
            if let Some(pre) = pre_i {
                if i - pre == 1 {
                    ans = max(ans, (nums[i] - nums[pre]).abs());
                } else {
                    update_ans(
                        min(nums[pre], nums[i]),
                        max(nums[pre], nums[i]),
                        i - pre > 2,
                    );
                }
            } else if i > 0 {
                update_ans(nums[i], nums[i], false);
            }
            pre_i = Some(i);
        }

        if let Some(pre) = pre_i {
            if pre < n - 1 {
                update_ans(nums[pre], nums[pre], false);
            }
        }

        ans
    }
}

fn main() {
    // Reading input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the number of elements (n)
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Parse the nums vector
    let nums: Vec<i32> = {
        lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };

    // Solution instance and result computation
    let sol = Solution {};
    let result = sol.min_difference(nums);

    // Print the result
    println!("{}", result);
}