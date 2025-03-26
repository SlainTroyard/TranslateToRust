use std::collections::HashSet;
use std::io::{self, BufRead, Write};

/// Computes the minimum number of operations needed as defined in the problem.
///
/// The algorithm iterates the array from back to front and uses a HashSet to track seen numbers.
/// When a duplicate is found, the function returns (i / 3) + 1, where i is the current index.
/// If no duplicate is found, it returns 0.
fn minimum_operations(nums: &[i32]) -> i32 {
    let mut seen = HashSet::new();
    // Iterate backwards over the indices and values.
    for (i, &num) in nums.iter().enumerate().rev() {
        // If the value is already in the set, return the computed result.
        if !seen.insert(num) {
            return (i as i32) / 3 + 1;
        }
    }
    0
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    // Use a locked iterator over lines from stdin.
    let mut lines = stdin.lock().lines();

    // Read the first line which contains the size of the array (n).
    let n_line = match lines.next() {
        Some(line) => line?,
        None => return Ok(()), // If no input is provided, exit gracefully.
    };

    let n: usize = n_line.trim().parse().expect("Failed to parse the size of the array (n)");

    // Prepare a vector to hold the input numbers.
    let mut nums = Vec::with_capacity(n);

    // Read numbers until we have exactly n integers.
    // This approach handles inputs that may have multiple values per line.
    while nums.len() < n {
        let line = match lines.next() {
            Some(line) => line?,
            None => break,
        };
        for token in line.split_whitespace() {
            if nums.len() >= n {
                break;
            }
            let num = token.parse::<i32>().expect("Failed to parse an integer");
            nums.push(num);
        }
    }

    // Check if we have exactly n numbers; if not, return an error.
    if nums.len() != n {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Insufficient numbers provided",
        ));
    }

    // Call the function with the provided numbers.
    let result = minimum_operations(&nums);

    // Output the result with a newline, matching the format of the original code.
    writeln!(io::stdout(), "{}", result)?;
    Ok(())
}