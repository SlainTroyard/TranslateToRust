use std::io;

/// Translates the C code from LeetCode Weekly Contest 429 Problem 1 to idiomatic Rust.
///
/// # Requirements
/// 1. Preserves the original algorithm logic.
/// 2. Uses idiomatic Rust with proper error handling.
/// 3. Maintains exact stdin/stdout format.
///
/// # Input Format
/// The first line contains an integer `n` (the number of elements).
/// The second line contains `n` space-separated integers.
///
/// # Output Format
/// Prints the minimum number of operations as an integer.
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let n = nums[0] as usize;
    let nums_slice = &nums[1..1 + n]; // Slice of the next `n` elements after the first

    let result = minimum_operations(nums_slice);
    println!("{}", result);
}

/// Calculates the minimum operations needed to remove duplicates by grouping elements in threes.
///
/// # Arguments
/// * `nums` - A slice of integers to process.
///
/// # Returns
/// The minimum number of operations required.
fn minimum_operations(nums: &[i32]) -> i32 {
    let mut count = [0; 101]; // Tracks occurrences of numbers 0-100
    for (i, &num) in nums.iter().enumerate().rev() {
        count[num as usize] += 1;
        if count[num as usize] > 1 {
            // Returns the ceiling of (i + 1) / 3 using integer division trick
            return (i as i32 + 3) / 3;
        }
    }
    0
}