use std::io;

/// Computes the maximum score based on the given array of integers.
///
/// The algorithm iterates through the array, tracking the maximum value encountered so far up to
/// each position (excluding the last element). The accumulated sum of these maximum values is the result.
pub fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans: i64 = 0;
    let mut current_max: i32 = 0;
    for i in 0..nums.len().saturating_sub(1) {
        current_max = current_max.max(nums[i]);
        ans += current_max as i64;
    }
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut tokens = input.split_whitespace();

    // Read the size of the array
    let size_str = tokens.next().expect("No size provided");
    let size: usize = size_str.parse().expect("Invalid size");

    // Parse the array elements and validate the count
    let nums: Vec<i32> = tokens
        .map(|s| s.parse().expect("Invalid number"))
        .take(size)
        .collect();

    if nums.len() != size {
        panic!("Not enough numbers provided");
    }

    // Compute and print the result
    let result = find_maximum_score(&nums);
    println!("{}", result);
}