use std::cmp::{max, min};
use std::io::{self, BufRead, Write};

/// This function calculates the minimum difference as described in
/// LeetCode Weekly Contest 424 Problem 4.
///
/// The algorithm iterates over the nums vector, identifies special boundaries
/// (elements adjacent to missing values denoted by -1), and uses a closure to
/// update the answer based on provided parameters.
///
fn min_difference(nums: &[i32]) -> i32 {
    let n = nums.len();
    // Initialize min_l to maximum possible i32 value and max_r to 0.
    let mut min_l = i32::MAX;
    let mut max_r = 0;

    // Determine the boundaries: elements that are not -1 but next to -1.
    for i in 0..n {
        // Check if the current number is not -1 and if either neighbour is -1.
        if nums[i] != -1 && ((i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1)) {
            min_l = min(min_l, nums[i]);
            max_r = max(max_r, nums[i]);
        }
    }

    let mut ans = 0;

    {
        // Define a closure that updates the answer based on the bounds l, r and a flag big.
        let mut update_ans = |l: i32, r: i32, big: bool| {
            // Calculate d as the half (rounded up) of the minimum of
            // (r - min_l) and (max_r - l).
            let d_initial = min(r - min_l, max_r - l);
            let mut d = (d_initial + 1) / 2;
            if big {
                // For longer gaps, d is the minimum between d and (max_r - min_l + 2)/3.
                d = min(d, (max_r - min_l + 2) / 3);
            }
            ans = max(ans, d);
        };

        // pre_i will hold the index of the previous valid (non -1) number.
        let mut pre_i: i32 = -1;
        for i in 0..n {
            if nums[i] == -1 {
                continue;
            }
            if pre_i >= 0 {
                if i - pre_i as usize == 1 {
                    // If consecutive valid numbers, update answer by their absolute difference.
                    ans = max(ans, (nums[i] - nums[pre_i as usize]).abs());
                } else {
                    // For non-consecutive valid numbers, update the answer using update_ans.
                    let l = nums[pre_i as usize].min(nums[i]);
                    let r = nums[pre_i as usize].max(nums[i]);
                    update_ans(l, r, i - pre_i as usize > 2);
                }
            } else if i > 0 {
                // Special case: if the first valid number is not at the very start,
                // call update_ans to properly adjust boundaries.
                update_ans(nums[i], nums[i], false);
            }
            pre_i = i as i32;
        }
        // If the last valid number is not at the end, update answer one last time.
        if pre_i >= 0 && (pre_i as usize) < n - 1 {
            update_ans(nums[pre_i as usize], nums[pre_i as usize], false);
        }
    }
    ans
}

fn main() -> io::Result<()> {
    // Use a buffered reader for efficient input reading.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    // Read the entire input.
    reader.read_to_string(&mut input)?;

    // Split the input by whitespace and create an iterator.
    let mut tokens = input.split_whitespace();

    // The first token is the number of elements.
    let n: usize = tokens
        .next()
        .expect("Expected number of elements")
        .parse()
        .expect("Failed to parse n");

    // Read n integers into the nums vector.
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num = tokens
            .next()
            .expect("Expected a number")
            .parse::<i32>()
            .expect("Failed to parse number");
        nums.push(num);
    }

    // Calculate the result using the min_difference function.
    let result = min_difference(&nums);

    // Write the result to stdout with a newline.
    let stdout = io::stdout();
    let mut out = stdout.lock();
    writeln!(out, "{result}")?;

    Ok(())
}