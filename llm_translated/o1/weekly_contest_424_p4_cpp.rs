use std::io::{self, BufRead};

/// A struct representing our solution, analogous to the C++ class Solution.
struct Solution;

impl Solution {
    /// Translated version of the C++ method:
    /// int minDifference(vector<int>& nums)
    /// This calculates the answer according to the original problem logic.
    fn min_difference(&self, nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();

        // Initialize min_l to i32::MAX (similar to INT_MAX) and max_r to 0 as in the original code
        let mut min_l = i32::MAX;
        let mut max_r = 0;

        // Detect values next to -1 and collect the min_l and max_r among them
        for i in 0..n {
            if nums[i] != -1
                && ((i > 0 && nums[i - 1] == -1) || (i + 1 < n && nums[i + 1] == -1))
            {
                min_l = min_l.min(nums[i]);
                max_r = max_r.max(nums[i]);
            }
        }

        let mut ans = 0;

        // This helper function updates the 'ans' value the same way the C++ lambda does
        fn update_ans(ans: &mut i32, l: i32, r: i32, big: bool, min_l: i32, max_r: i32) {
            // d = (min(r - min_l, max_r - l) + 1) / 2
            let gap = (r - min_l).min(max_r - l);
            let mut d = (gap + 1) / 2;

            // If 'big' is true, apply the additional constraint
            if big {
                // d = min(d, (max_r - min_l + 2) / 3)
                d = d.min((max_r - min_l + 2) / 3);
            }
            // ans = max(ans, d)
            *ans = (*ans).max(d);
        }

        // Track the index of the previously valid element
        let mut pre_i: isize = -1;

        // Iterate through nums, skipping -1 values, and update ans according to the logic
        for i in 0..n {
            if nums[i] == -1 {
                continue;
            }
            if pre_i >= 0 {
                let gap = i as isize - pre_i;
                if gap == 1 {
                    // ans = max(ans, abs(nums[i] - nums[pre_i]))
                    ans = ans.max((nums[i] - nums[pre_i as usize]).abs());
                } else {
                    // update_ans(min(nums[pre_i], nums[i]), max(nums[pre_i], nums[i]), gap > 2)
                    let l = nums[pre_i as usize].min(nums[i]);
                    let r = nums[pre_i as usize].max(nums[i]);
                    update_ans(&mut ans, l, r, gap > 2, min_l, max_r);
                }
            } else if i > 0 {
                // update_ans(nums[i], nums[i], false)
                update_ans(&mut ans, nums[i], nums[i], false, min_l, max_r);
            }
            pre_i = i as isize;
        }

        // If there was at least one valid index and it's not the last one, call update_ans again
        if pre_i >= 0 && (pre_i as usize) < n - 1 {
            // update_ans(nums[pre_i], nums[pre_i], false);
            let val = nums[pre_i as usize];
            update_ans(&mut ans, val, val, false, min_l, max_r);
        }

        ans
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read all lines from stdin
    let stdin = io::stdin();
    let lines = stdin.lock().lines().collect::<Result<Vec<_>, _>>()?;

    // Split all lines by whitespace into tokens
    let tokens: Vec<String> = lines
        .iter()
        .flat_map(|line| line.split_whitespace().map(String::from))
        .collect();

    // Parse n
    let mut idx = 0;
    let n = tokens[idx].parse::<usize>()?;
    idx += 1;

    // Parse the next n integers into nums
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let val = tokens[idx].parse::<i32>()?;
        nums.push(val);
        idx += 1;
    }

    // Create our solution instance and compute the result
    let sol = Solution;
    let result = sol.min_difference(&mut nums);

    // Output the result as the original code does
    println!("{}", result);

    Ok(())
}