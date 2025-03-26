use std::io;
use std::cmp;
use std::error::Error;

fn min_difference(nums: &[i32]) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }

    let mut min_l = i32::MAX;
    let mut max_r = 0;

    // Find min_l and max_r by checking for the adjacent '-1' elements
    for i in 0..n {
        if nums[i] != -1 && 
           ((i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1)) {
            if nums[i] < min_l {
                min_l = nums[i];
            }
            if nums[i] > max_r {
                max_r = nums[i];
            }
        }
    }

    if min_l == i32::MAX {
        return 0;
    }

    let mut ans = 0;

    // Helper closure to update the result
    let mut update_ans = |l: i32, r: i32, big: bool| {
        let d = if (r - min_l) < (max_r - l) {
            r - min_l
        } else {
            max_r - l
        } + 1;
        let d = d / 2;
        let mut new_d = d;
        if big {
            let candidate = (max_r - min_l + 2) / 3;
            new_d = cmp::min(d, candidate);
        }
        if new_d > ans {
            ans = new_d;
        }
    };

    let mut pre_i = -1;
    for i in 0..n {
        if nums[i] == -1 {
            continue;
        }
        if pre_i >= 0 {
            if i - pre_i == 1 {
                let diff = (nums[i] - nums[pre_i]).abs();
                if diff > ans {
                    ans = diff;
                }
            } else {
                let l = cmp::min(nums[pre_i], nums[i]);
                let r = cmp::max(nums[pre_i], nums[i]);
                update_ans(l, r, i - pre_i > 2);
            }
        } else if i > 0 {
            update_ans(nums[i], nums[i], false);
        }
        pre_i = i as i32;
    }

    if pre_i >= 0 && pre_i < (n - 1) as i32 {
        let idx = pre_i as usize;
        update_ans(nums[idx], nums[idx], false);
    }

    ans
}

fn read_input() -> Result<Vec<i32>, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<_, _>>()?;
    Ok(nums)
}

fn main() -> Result<(), Box<dyn Error>> {
    let nums = read_input()?;
    if nums.is_empty() {
        println!("0");
        return Ok(());
    }
    let result = min_difference(&nums);
    println!("{}", result);
    Ok(())
}