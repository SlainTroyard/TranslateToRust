use std::io::{self, Write};  // For input/output
use std::str::FromStr;      // For parsing integers from strings

// Function to count partitions as per the algorithm
fn count_partitions(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut count = 0;
    for i in 0..nums_size - 1 {
        let left_sum: i32 = nums[..=i].iter().sum();  // Sum of elements from 0 to i
        let right_sum: i32 = nums[i + 1..].iter().sum();  // Sum of elements from i+1 to end
        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut input = String::new();

    // Read the size of the array (n)
    io::stdin().read_line(&mut input).unwrap();
    let n = usize::from_str(input.trim()).unwrap();
    input.clear();

    // Read the array `nums`
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|num| i32::from_str(num).unwrap())
        .collect();

    // Ensure the input matches the expected length, as in C
    assert_eq!(nums.len(), n, "The input size does not match the expected size!");

    // Call the function and print the result
    let result = count_partitions(&nums);
    writeln!(io::stdout(), "{}", result).unwrap();
}