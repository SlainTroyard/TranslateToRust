use std::io::{self, BufRead};
use std::cmp::{max, min};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line which contains the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the second line which contains the array elements
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = min_difference(nums);

    println!("{}", result);
}

fn min_difference(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut min_l = i32::MAX;
    let mut max_r = 0;

    // Find the minimum and maximum values adjacent to -1
    for i in 0..n {
        if nums[i] != -1 && (i > 0 && nums[i - 1] == -1 || i < n - 1 && nums[i + 1] == -1) {
            min_l = min(min_l, nums[i]);
            max_r = max(max_r, nums[i]);
        }
    }

    let mut ans = 0;

    // Closure to update the answer based on the given range
    let mut update_ans = |l: i32, r: i32, big: bool| {
        let d = (min(r - min_l, max_r - l) + 1) / 2;
        let d = if big {
            min(d, (max_r - min_l + 2) / 3)
        } else {
            d
        };
        ans = max(ans, d);
    };

    let mut pre_i = -1;

    // Iterate through the array to calculate the maximum difference
    for i in 0..n {
        if nums[i] == -1 {
            continue;
        }
        if pre_i >= 0 {
            if i - pre_i == 1 {
                ans = max(ans, (nums[i] - nums[pre_i as usize]).abs());
            } else {
                update_ans(min(nums[pre_i as usize], nums[i]), max(nums[pre_i as usize], nums[i]), i - pre_i as usize > 2);
            }
        } else if i > 0 {
            update_ans(nums[i], nums[i], false);
        }
        pre_i = i as i32;
    }

    if 0 <= pre_i && pre_i < n as i32 - 1 {
        update_ans(nums[pre_i as usize], nums[pre_i as usize], false);
    }

    ans
}