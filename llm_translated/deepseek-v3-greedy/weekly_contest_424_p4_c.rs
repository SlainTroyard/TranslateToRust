use std::io::{self, BufRead};
use std::cmp::{min, max};

// Function to calculate the minimum difference as in the C++ solution
fn min_difference(nums: &[i32]) -> i32 {
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
    let mut update_ans = |l: i32, r: i32, big: bool| {
        let mut d = (if r - min_l < max_r - l { r - min_l } else { max_r - l }) + 1;
        d /= 2;
        if big {
            d = min(d, (max_r - min_l + 2) / 3);
        }
        if d > ans { ans = d; }
    };

    // Calculate the answer by iterating through the elements
    let mut pre_i = -1;
    for i in 0..n {
        if nums[i] == -1 {
            continue;
        }
        if pre_i >= 0 {
            if i as i32 - pre_i == 1 {
                let diff = (nums[i] - nums[pre_i as usize]).abs();
                if diff > ans { ans = diff; }
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

    if 0 <= pre_i && pre_i < n as i32 - 1 {
        update_ans(nums[pre_i as usize], nums[pre_i as usize], false);
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line to get the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the second line to get the array elements
    let nums_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate the result
    let result = min_difference(&nums);

    // Print the result
    println!("{}", result);
}