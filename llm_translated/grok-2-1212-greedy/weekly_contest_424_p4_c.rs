use std::io::{self, BufRead};
use std::cmp::{min, max};

fn min_difference(nums: &mut [i32]) -> i32 {
    let n = nums.len();
    let mut min_l = i32::MAX;
    let mut max_r = 0;

    // Find min_l and max_r by checking for the adjacent '-1' elements
    for i in 0..n {
        if nums[i] != -1 && 
           ((i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1)) {
            if nums[i] < min_l { min_l = nums[i]; }
            if nums[i] > max_r { max_r = nums[i]; }
        }
    }

    let mut ans = 0;

    // Helper function to update the result (same logic as the C++ lambda function)
    fn update_ans(l: i32, r: i32, min_l: i32, max_r: i32, big: bool) -> i32 {
        let mut d = if r - min_l < max_r - l { r - min_l } else { max_r - l } + 1;
        d /= 2;
        if big {
            d = if d < (max_r - min_l + 2) / 3 { d } else { (max_r - min_l + 2) / 3 };
        }
        d
    }

    // Calculate the answer by iterating through the elements
    let mut pre_i = -1;
    for i in 0..n {
        if nums[i] == -1 {
            continue;
        }
        if pre_i >= 0 {
            if i - pre_i == 1 {
                let diff = (nums[i] - nums[pre_i]).abs();
                if diff > ans { ans = diff; }
            } else {
                let l = min(nums[pre_i], nums[i]);
                let r = max(nums[pre_i], nums[i]);
                let new_ans = update_ans(l, r, min_l, max_r, i - pre_i > 2);
                if new_ans > ans { ans = new_ans; }
            }
        } else if i > 0 {
            let new_ans = update_ans(nums[i], nums[i], min_l, max_r, false);
            if new_ans > ans { ans = new_ans; }
        }
        pre_i = i as i32;
    }

    if pre_i >= 0 && pre_i < n as i32 - 1 {
        let new_ans = update_ans(nums[pre_i as usize], nums[pre_i as usize], min_l, max_r, false);
        if new_ans > ans { ans = new_ans; }
    }

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the array
    let mut nums: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate the result
    let result = min_difference(&mut nums);

    // Print the result
    println!("{}", result);

    Ok(())
}