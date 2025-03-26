use std::cmp::{max, min};
use std::io::{self, Read};

/// Calculate the minimum difference based on the given algorithm
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

    // Helper closure to update the result (same logic as the C update_ans function)
    let mut update_ans = |l: i32, r: i32, big: bool| {
        let mut d = min(r - min_l, max_r - l) + 1;
        d /= 2;
        if big {
            d = min(d, (max_r - min_l + 2) / 3);
        }
        ans = max(ans, d);
    };

    // Calculate the answer by iterating through the elements
    let mut pre_i: Option<usize> = None;
    for i in 0..n {
        if nums[i] == -1 {
            continue;
        }
        if let Some(prev) = pre_i {
            if i - prev == 1 {
                let diff = (nums[i] - nums[prev]).abs();
                ans = max(ans, diff);
            } else {
                update_ans(
                    min(nums[prev], nums[i]),
                    max(nums[prev], nums[i]),
                    i - prev > 2
                );
            }
        } else if i > 0 {
            update_ans(nums[i], nums[i], false);
        }
        pre_i = Some(i);
    }

    if let Some(prev) = pre_i {
        if prev < n - 1 {
            update_ans(nums[prev], nums[prev], false);
        }
    }

    ans
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Failed to parse n");
    
    let mut nums = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Parse space-separated integers
    nums = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Calculate and print result
    let result = min_difference(&nums);
    println!("{}", result);
}