use std::io::{self, Write};
use std::collections::HashSet;

/// Finds the first two duplicate numbers in the `nums` array (same logic as C code).
/// 
/// # Arguments
/// 
/// * `nums` - A slice of integers to search for duplicates.
/// 
/// # Returns
/// 
/// A Vec containing the first two duplicate numbers, if found. Always returns exactly 2 elements as per problem requirements.
fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(2);
    let mut seen = HashSet::new();

    for &num in nums {
        // If we have already seen this number, it's a duplicate
        if seen.contains(&num) {
            result.push(num);
            if result.len() == 2 {
                break;
            }
        } else {
            // Mark the number as seen
            seen.insert(num);
        }
    }

    // Ensure the result always contains 2 elements (based on problem requirements)
    result
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();

    // Reading the input size which is provided as a single integer.
    stdin.read_line(&mut input)?;
    let num_size: usize = input.trim().parse::<usize>().expect("Invalid input for array size");

    // Allocate `nums` array with size `num_size + 2`
    let mut nums = Vec::new();
    nums.reserve(num_size + 2);
    input.clear();

    // Read the next `num_size + 2` integers from stdin
    stdin.read_line(&mut input)?;
    nums = input
        .split_whitespace()
        .take(num_size + 2)
        .map(|x| x.parse::<i32>().expect("Invalid integer in input"))
        .collect();

    // Calculate duplicates using the function
    let result = get_sneaky_numbers(&nums);

    // Print the result
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    for num in result {
        write!(out, "{} ", num)?;
    }
    writeln!(out)?;

    Ok(())
}