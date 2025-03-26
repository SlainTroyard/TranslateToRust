use std::io::{self, BufRead};

/// Translates the C function `minDifference` exactly,
/// preserving its logic for computing the desired answer.
fn min_difference(nums: &mut [i32], n: usize) -> i32 {
    use std::cmp::{min, max};
    let mut min_l = i32::MAX;
    let mut max_r = 0;

    // Find min_l and max_r by checking for the adjacent '-1' elements
    for i in 0..n {
        let x = nums[i];
        if x != -1
            && ((i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1))
        {
            if x < min_l {
                min_l = x;
            }
            if x > max_r {
                max_r = x;
            }
        }
    }

    let mut ans = 0;

    // Helper closure to update the result (mirroring the C code's logic).
    let mut update_ans = |l: i32, r: i32, big: bool| {
        // Equivalent to: int d = (r - min_l < max_r - l ? r - min_l : max_r - l) + 1; d /= 2;
        let d_base = if (r - min_l) < (max_r - l) {
            r - min_l
        } else {
            max_r - l
        };
        let mut d = d_base + 1;
        d /= 2;

        // If 'big' is true, we take the minimum with (max_r - min_l + 2) / 3
        if big {
            let alt = (max_r - min_l + 2) / 3;
            if d > alt {
                d = alt;
            }
        }

        if d > ans {
            ans = d;
        }
    };

    // Calculate the answer by iterating through the elements
    let mut pre_i: i32 = -1;
    for i in 0..n {
        if nums[i] == -1 {
            continue;
        }
        if pre_i >= 0 {
            // If this is an adjacent index, compare difference directly
            if i as i32 - pre_i == 1 {
                let diff = (nums[i] - nums[pre_i as usize]).abs();
                if diff > ans {
                    ans = diff;
                }
            } else {
                // Otherwise, call update_ans for the pair
                let l = min(nums[pre_i as usize], nums[i]);
                let r = max(nums[pre_i as usize], nums[i]);
                let big = (i as i32 - pre_i) > 2;
                update_ans(l, r, big);
            }
        } else if i > 0 {
            // First non--1 element is not at index 0, call update_ans for that single value
            update_ans(nums[i], nums[i], false);
        }
        pre_i = i as i32;
    }

    // If the last non--1 element is not at the last position, update_ans again
    if pre_i >= 0 && (pre_i as usize) < n - 1 {
        let val = nums[pre_i as usize];
        update_ans(val, val, false);
    }

    ans
}

/// Main function mirroring the C code's I/O handling exactly.
/// Reads 'n', then reads 'n' integers (possibly split across lines),
/// calls the logic, and prints the result.
fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();

    // Read n
    stdin.read_line(&mut input_line).unwrap();
    let n: usize = input_line.trim().parse().unwrap();
    let mut nums = Vec::with_capacity(n);

    // Read n integers, regardless of how they are split across lines
    let mut count = 0;
    while count < n {
        input_line.clear();
        if stdin.read_line(&mut input_line).unwrap() == 0 {
            break; // No more input
        }
        for val_str in input_line.split_whitespace() {
            if count < n {
                let val: i32 = val_str.parse().unwrap();
                nums.push(val);
                count += 1;
            } else {
                break;
            }
        }
    }

    // Compute and print the result
    let result = min_difference(&mut nums, n);
    println!("{}", result);
}