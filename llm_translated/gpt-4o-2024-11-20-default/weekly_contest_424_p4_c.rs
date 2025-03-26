use std::io;
use std::cmp::{min, max};
use std::usize::MAX as USIZE_MAX;

fn min_difference(nums: &[i32]) -> i32 {
    let mut min_l = i32::MAX;
    let mut max_r = i32::MIN;

    let n = nums.len();

    // Finding min_l and max_r by checking adjacent `-1` elements
    for i in 0..n {
        if nums[i] != -1
            && ((i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1))
        {
            if nums[i] < min_l {
                min_l = nums[i];
            }
            if nums[i] > max_r {
                max_r = nums[i];
            }
        }
    }

    let mut ans = 0;

    // Helper function that updates the answer based on the provided logic
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

    // Calculating the answer by iterating over the elements
    let mut pre_i = None; // Previous index of a non `-1` element
    for i in 0..n {
        if nums[i] == -1 {
            continue;
        }

        if let Some(pi) = pre_i {
            if i - pi == 1 {
                let diff = (nums[i] - nums[pi]).abs();
                ans = max(ans, diff);
            } else {
                update_ans(
                    min(nums[pi], nums[i]),
                    max(nums[pi], nums[i]),
                    i - pi > 2,
                );
            }
        } else if i > 0 {
            update_ans(nums[i], nums[i], false);
        }

        pre_i = Some(i);
    }

    if let Some(pi) = pre_i {
        if pi < n - 1 {
            update_ans(nums[pi], nums[pi], false);
        }
    }

    ans
}

fn main() {
    // Reading input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let n: usize = input.trim().parse().expect("Expected an integer");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read nums input");

    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Expected an integer"))
        .collect();

    assert_eq!(nums.len(), n, "Input length mismatch");

    // Calling the solution function
    let result = min_difference(&nums);

    // Printing the result
    println!("{}", result);
}