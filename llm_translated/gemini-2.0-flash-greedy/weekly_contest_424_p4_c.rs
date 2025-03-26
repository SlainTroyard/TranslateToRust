use std::io;
use std::cmp::{min, max, abs};

fn min_difference(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut min_l = i32::MAX;
    let mut max_r = 0;

    // Find min_l and max_r by checking for the adjacent '-1' elements
    for i in 0..n {
        if nums[i] != -1 &&
            ((i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1)) {
            min_l = min(min_l, nums[i]);
            max_r = max(max_r, nums[i]);
        }
    }

    let mut ans = 0;

    // Helper function to update the result (same logic as the C++ lambda function)
    let mut update_ans = |l: i32, r: i32, big: bool| {
        let mut d = min(r - min_l, max_r - l) + 1;
        d /= 2;
        if big {
            d = min(d, (max_r - min_l + 2) / 3);
        }
        ans = max(ans, d);
    };

    // Calculate the answer by iterating through the elements
    let mut pre_i: i32 = -1;
    for i in 0..n {
        if nums[i] == -1 {
            continue;
        }
        if pre_i >= 0 {
            if i as i32 - pre_i == 1 {
                let diff = abs(nums[i] - nums[pre_i as usize]);
                ans = max(ans, diff);
            } else {
                update_ans(min(nums[pre_i as usize], nums[i]),
                           max(nums[pre_i as usize], nums[i]),
                           i as i32 - pre_i > 2);
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

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Invalid input");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let result = min_difference(&nums);
    println!("{}", result);
}