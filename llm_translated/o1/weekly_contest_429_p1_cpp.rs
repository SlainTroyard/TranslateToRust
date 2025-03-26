use std::collections::HashSet;
use std::io::{self, BufRead};

/// Returns the minimum number of operations as defined by the problem statement.
///
/// The logic is to iterate from the end of the array to the beginning,
/// and return (i / 3 + 1) upon encountering the first repeated element.
fn minimum_operations(nums: &[i32]) -> i32 {
    let mut seen = HashSet::new();
    // Iterate from the last index to the first
    for (i, &value) in nums.iter().enumerate().rev() {
        // Attempt to insert the value into the set;
        // if insertion fails, it means it's already present
        if !seen.insert(value) {
            // Return i / 3 + 1 as soon as we find a duplicate
            return (i as i32) / 3 + 1;
        }
    }
    // If no duplicates found, the result is 0
    0
}

fn main() -> io::Result<()> {
    // Prepare to read from standard input
    let stdin = io::stdin();
    let mut line = String::new();

    // Read the integer n from stdin
    stdin.read_line(&mut line)?;
    let n: usize = line.trim().parse().expect("Failed to parse n");

    // Read exactly n integers, which may be spread across multiple lines
    let mut nums = Vec::with_capacity(n);
    let mut count = 0;
    while count < n {
        line.clear();
        let bytes_read = stdin.read_line(&mut line)?;
        if bytes_read == 0 {
            break; // no more input
        }
        // Split the current line on whitespace, and parse each token as i32
        for token in line.split_whitespace() {
            if let Ok(value) = token.parse::<i32>() {
                nums.push(value);
                count += 1;
                if count == n {
                    break;
                }
            }
        }
    }

    // Compute and print the result
    let result = minimum_operations(&nums);
    println!("{}", result);

    Ok(())
}