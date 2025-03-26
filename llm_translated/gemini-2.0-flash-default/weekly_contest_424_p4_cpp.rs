use std::cmp::{max, min};
use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min_l = i32::MAX;
        let mut max_r = 0;

        // Find the minimum value to the left of -1 and the maximum value to the right of -1
        for i in 0..n {
            if nums[i] != -1
                && ((i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1))
            {
                min_l = min(min_l, nums[i]);
                max_r = max(max_r, nums[i]);
            }
        }

        let mut ans = 0;

        // Define a helper closure to update the answer
        let mut update_ans = |l: i32, r: i32, big: bool| {
            let d = (min(r - min_l, max_r - l) + 1) / 2;
            let mut d = d;
            if big {
                d = min(d, (max_r - min_l + 2) / 3);
            }
            ans = max(ans, d);
        };

        let mut pre_i = -1;

        for i in 0..n {
            if nums[i] == -1 {
                continue;
            }
            if pre_i >= 0 {
                if i as i32 - pre_i == 1 {
                    ans = max(ans, (nums[i] - nums[pre_i as usize]).abs());
                } else {
                    update_ans(
                        min(nums[pre_i as usize], nums[i]),
                        max(nums[pre_i as usize], nums[i]),
                        i as i32 - pre_i > 2,
                    );
                }
            } else if i > 0 {
                update_ans(nums[i], nums[i], false);
            }
            pre_i = i as i32;
        }

        if 0 <= pre_i && pre_i < (n as i32 - 1) {
            update_ans(nums[pre_i as usize], nums[pre_i as usize], false);
        }

        ans
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n = lines.next().unwrap().parse::<usize>().unwrap();

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let sol = Solution {};
    let result = sol.min_difference(nums);

    println!("{}", result);

    Ok(())
}