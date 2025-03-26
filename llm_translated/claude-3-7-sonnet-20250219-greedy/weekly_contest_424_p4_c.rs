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
            min_l = min(min_l, nums[i]);
            max_r = max(max_r, nums[i]);
        }
    }

    let mut ans = 0;

    // Helper function to update the result (same logic as the C++ lambda function)
    let mut update_ans = |l: i32, r: i32, big: bool| {
        let d = min(r - min_l, max_r - l) + 1;
        let d = d / 2;
        let d = if big {
            min(d, (max_r - min_l + 2) / 3)
        } else {
            d
        };
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
                let diff = (nums[i] - nums[pre_i as usize]).abs();
                ans = max(ans, diff);
            } else {
                update_ans(
                    min(nums[pre_i as usize], nums[i]),
                    max(nums[pre_i as usize], nums[i]),
                    i as i32 - pre_i > 2
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read nums array
    let nums: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let result = min_difference(&nums);
    println!("{}", result);
    
    Ok(())
}