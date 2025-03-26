use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn min_difference(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min_l = i32::MAX;
        let mut max_r = 0;

        for i in 0..n {
            if nums[i] != -1 && (i > 0 && nums[i - 1] == -1 || i < n - 1 && nums[i + 1] == -1) {
                min_l = min_l.min(nums[i]);
                max_r = max_r.max(nums[i]);
            }
        }

        let mut ans = 0;

        let update_ans = |l: i32, r: i32, big: bool| {
            let mut d = (r - min_l).min(max_r - l) / 2 + 1;
            if big {
                d = d.min((max_r - min_l + 2) / 3);
            }
            ans = ans.max(d);
        };

        let mut pre_i = -1;
        for i in 0..n {
            if nums[i] == -1 {
                continue;
            }
            if pre_i >= 0 {
                if i - pre_i == 1 {
                    ans = ans.max((nums[i] - nums[pre_i]).abs());
                } else {
                    update_ans(nums[pre_i].min(nums[i]), nums[pre_i].max(nums[i]), i - pre_i > 2);
                }
            } else if i > 0 {
                update_ans(nums[i], nums[i], false);
            }
            pre_i = i as i32;
        }
        if pre_i >= 0 && pre_i < n as i32 - 1 {
            update_ans(nums[pre_i as usize], nums[pre_i as usize], false);
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the vector elements
    let mut nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Ensure the vector has the correct length
    assert_eq!(nums.len(), n);

    let result = Solution::min_difference(&mut nums);

    // Print the result
    println!("{}", result);

    Ok(())
}