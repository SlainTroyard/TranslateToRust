// Weekly Contest 433 Problem 4 in Rust
//
// This program reads two integers n and k from stdin,
// then reads an array of n integers. It computes and prints
// the result of the minMaxSubarraySum logic from the original C++ code.
//
// To run:
// 1) Compile: rustc this_file.rs
// 2) Run:     ./this_file
// 3) Provide input via stdin in the same format as the C++ code:
//        n k
//        <n space-separated integers>

use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the first line, which contains n and k
    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line)?;
    let first_parts: Vec<i64> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = first_parts[0] as usize;
    let k = first_parts[1];

    // Read the second line, which contains n integers
    let mut second_line = String::new();
    io::stdin().read_line(&mut second_line)?;
    let mut nums: Vec<i64> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    nums.truncate(n); // Ensure we only take the first n numbers

    // Compute the result using the translated logic
    let ans = min_max_subarray_sum(&mut nums, k);

    // Print the result
    println!("{}", ans);

    Ok(())
}

// This function replicates the core logic in the original C++ class `Solution::minMaxSubarraySum`.
fn min_max_subarray_sum(nums: &mut Vec<i64>, k: i64) -> i64 {
    // We push an extra sentinel value (INT_MIN / 2) onto nums
    // to force the stack-clearing mechanism at the end.
    nums.push((std::i32::MIN as i64) / 2);

    // Invoke sum_subarray_mins (computes sum of minimums of subarrays
    // up to length k, based on the "count" logic).
    let mut ans = sum_subarray_mins(nums, k);

    // Convert all elements in nums to their negation
    for x in nums.iter_mut() {
        *x = -*x;
    }

    // Part of the original logic: flip the last pushed sentinel again
    if let Some(last) = nums.last_mut() {
        *last = -*last;
    }

    // Subtract the sum of (negated) subarray mins
    ans -= sum_subarray_mins(nums, k);

    ans
}

// This function computes the sum of subarray minimums, assisted by a monotonic stack.
// It uses the "count" function to handle the counting logic for subarrays of length up to k.
fn sum_subarray_mins(nums: &Vec<i64>, k: i64) -> i64 {
    let mut res: i64 = 0;
    let mut st: Vec<i32> = Vec::new();
    st.push(-1); // The original C++ uses st.push(-1) as a sentinel

    for r in 0..nums.len() {
        // Pop from stack while current element is smaller than or equal to the top,
        // ensuring a strictly increasing stack of values.
        while st.len() > 1 && nums[st[st.len() - 1] as usize] >= nums[r] {
            let i = st.pop().unwrap();
            let l = *st.last().unwrap();
            let cnt = count((r as i64) - (l as i64) - 1, k)
                - count((i as i64) - (l as i64) - 1, k)
                - count((r as i64) - (i as i64) - 1, k);
            res += nums[i as usize] * cnt;
        }
        st.push(r as i32);
    }

    res
}

// Helper function "count" from the original C++ lambda
// Counts how many subarrays (up to length k) include this particular element.
fn count(m: i64, k: i64) -> i64 {
    if m > k {
        ((m * 2 - k + 1) * k) / 2
    } else {
        ((m + 1) * m) / 2
    }
}