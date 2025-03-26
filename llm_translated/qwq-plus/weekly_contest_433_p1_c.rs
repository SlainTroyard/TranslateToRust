use std::io;

/// Computes the sum of subarrays as per the problem's logic.
///
/// # Arguments
///
/// * `nums` - A slice of integers representing the input array.
///
/// # Returns
///
/// * The computed result as an i32.
fn subarray_sum(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut sums = vec![0; n + 1];
    let mut ans = 0;
    for i in 0..n {
        sums[i + 1] = sums[i] + nums[i];
        // Calculate the starting index using max(0, i - nums[i]) logic
        let start = (i as isize - nums[i] as isize).max(0) as usize;
        ans += sums[i + 1] - sums[start];
    }
    ans
}

/// Main function handling input/output and invoking the algorithm.
///
/// # Input Format
///
/// The first line contains an integer `n`, followed by `n` space-separated integers.
///
/// # Output Format
///
/// A single integer representing the result of `subarray_sum`.
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let n = iter.next().unwrap() as usize;
    let nums: Vec<i32> = iter.take(n).collect();
    println!("{}", subarray_sum(&nums));
}