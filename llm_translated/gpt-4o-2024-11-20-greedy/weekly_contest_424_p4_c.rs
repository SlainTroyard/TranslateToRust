use std::io::{self, BufRead};
use std::cmp::{min, max};
use std::collections::VecDeque;

fn min_difference(nums: &[i32], n: usize) -> i32 {
    let mut min_l = i32::MAX;
    let mut max_r = 0;

    // Find `min_l` and `max_r` by checking adjacent `-1` elements
    for i in 0..n {
        if nums[i] != -1
            && ((i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1))
        {
            min_l = min(min_l, nums[i]);
            max_r = max(max_r, nums[i]);
        }
    }

    let mut ans = 0;

    // Helper function to update `ans`
    let mut update_ans = |l: i32, r: i32, big: bool| {
        let mut d = if r - min_l < max_r - l {
            r - min_l
        } else {
            max_r - l
        } + 1;
        d /= 2;
        if big {
            d = min(d, (max_r - min_l + 2) / 3);
        }
        ans = max(ans, d);
    };

    // Calculate the answer by iterating over the elements
    let mut pre_i = -1;
    for i in 0..n {
        if nums[i] == -1 {
            continue;
        }
        if pre_i >= 0 {
            if i as i32 - pre_i == 1 {
                let diff = (nums[i] - nums[pre_i as usize]).abs();
                ans = max(ans, diff);
            } else {
                let l = min(nums[pre_i as usize], nums[i]);
                let r = max(nums[pre_i as usize], nums[i]);
                update_ans(l, r, i as i32 - pre_i > 2);
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

fn main() {
    // Read input via stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line contains the number of elements `n`
    let n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse `n`");

    // Second line contains the array `nums`
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    if nums.len() != n {
        panic!("Input array length does not match `n`");
    }

    // Call the `min_difference` function and print the result
    let result = min_difference(&nums, n);
    println!("{}", result);
}