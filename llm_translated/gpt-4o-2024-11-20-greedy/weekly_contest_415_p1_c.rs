use std::io::{self, Write};
use std::collections::HashSet;

/// Finds the first two duplicate numbers in the input array.
/// Returns a vector containing the two numbers.
fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(2);
    let mut seen = HashSet::new();

    for &num in nums {
        if seen.contains(&num) {
            result.push(num);
            if result.len() == 2 {
                break;
            }
        } else {
            seen.insert(num);
        }
    }

    result
}

fn main() {
    // Read the input size from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_size: usize = input.trim().parse().expect("Invalid input");

    // Adjust the size to include the extra 2 numbers
    let adjusted_size = num_size + 2;

    // Read the array of numbers from stdin
    let mut nums = Vec::with_capacity(adjusted_size);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    nums.extend(
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Invalid number")),
    );

    // Ensure the input size matches the expected adjusted size
    assert_eq!(nums.len(), adjusted_size, "Input size does not match expected size");

    // Get the sneaky numbers
    let result = get_sneaky_numbers(&nums);

    // Print the result
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for (i, num) in result.iter().enumerate() {
        if i > 0 {
            write!(handle, " ").expect("Failed to write output");
        }
        write!(handle, "{}", num).expect("Failed to write output");
    }
    writeln!(handle).expect("Failed to write output");
}