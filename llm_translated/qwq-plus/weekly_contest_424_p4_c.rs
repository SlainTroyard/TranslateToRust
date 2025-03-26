use std::io;

/// Computes the minimum difference as per the problem's logic.
///
/// # Arguments
///
/// * `nums` - A slice of integers representing the input array.
///
/// # Returns
///
/// The computed result as an integer.
fn min_difference(nums: &[i32]) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }

    let mut min_l = i32::MAX;
    let mut max_r = 0;

    // Find min_l and max_r by checking elements adjacent to -1
    for i in 0..n {
        let num = nums[i];
        if num != -1 {
            let has_prev = i > 0 && nums[i - 1] == -1;
            let has_next = i < n - 1 && nums[i + 1] == -1;
            if has_prev || has_next {
                if num < min_l {
                    min_l = num;
                }
                if num > max_r {
                    max_r = num;
                }
            }
        }
    }

    let mut ans = 0;

    // Helper closure to compute the value based on l, r, and big flag
    let compute_d = |l: i32, r: i32, big: bool| -> i32 {
        let d_val = if r - min_l < max_r - l {
            r - min_l
        } else {
            max_r - l
        } + 1;
        let mut d = d_val / 2;
        if big {
            let third = (max_r - min_l + 2) / 3;
            if d > third {
                d = third;
            }
        }
        d
    };

    let mut pre_i = -1isize;

    for i in 0..n {
        if nums[i] == -1 {
            continue;
        }

        if pre_i >= 0 {
            let distance = i as isize - pre_i;
            if distance == 1 {
                let diff = (nums[i] - nums[pre_i as usize]).abs();
                if diff > ans {
                    ans = diff;
                }
            } else {
                let l = nums[pre_i as usize].min(nums[i]);
                let r = nums[pre_i as usize].max(nums[i]);
                let big = distance > 2;
                let d = compute_d(l, r, big);
                ans = ans.max(d);
            }
            pre_i = i as isize;
        } else {
            if i > 0 {
                let l = nums[i];
                let d = compute_d(l, l, false);
                ans = ans.max(d);
            }
            pre_i = i as isize;
        }
    }

    // Final check for the last element if applicable
    if pre_i >= 0 {
        let idx = pre_i as usize;
        if idx < n - 1 {
            let l = nums[idx];
            let d = compute_d(l, l, false);
            ans = ans.max(d);
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");

    let mut iter = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid integer in input"));

    let n = iter.next().expect("No value for n provided");
    let nums: Vec<i32> = iter.by_ref().take(n as usize).collect();

    if nums.len() != n as usize {
        panic!(
            "Expected {} numbers, but got {}",
            n,
            nums.len()
        );
    }

    let result = min_difference(&nums);
    println!("{}", result);
}