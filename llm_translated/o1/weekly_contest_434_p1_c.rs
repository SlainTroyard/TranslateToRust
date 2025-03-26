// Weekly Contest 434 Problem 1
// Translated from C to Rust with idiomatic error handling.
// This program reads an integer n, then reads n integers,
// then outputs the result of the count_partitions function.

use std::error::Error;
use std::io::{self, BufRead};

/// Returns the number of partitions (from the problem statement)
/// for which (leftSum - rightSum) % 2 == 0.
fn count_partitions(nums: &[i32]) -> i32 {
    let mut count = 0;
    let n = nums.len();

    // For each possible partition point i
    for i in 0..n - 1 {
        let mut left_sum = 0;
        let mut right_sum = 0;

        // Sum all elements up to and including i
        for j in 0..=i {
            left_sum += nums[j];
        }

        // Sum all elements after i
        for j in i + 1..n {
            right_sum += nums[j];
        }

        // Check if (leftSum - rightSum) is even
        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }
    }

    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the integer n
    let line_for_n = lines
        .next()
        .ok_or("Expected input for n but none was found")??;
    let n: usize = line_for_n.trim().parse()?;

    // Prepare to read n integers
    let mut nums = Vec::with_capacity(n);
    let mut read_count = 0;

    // Read from subsequent lines/whitespace-terminated values until we get n integers
    'outer: while read_count < n {
        let line = match lines.next() {
            Some(Ok(l)) => l,
            Some(Err(e)) => return Err(e.into()),
            None => break 'outer, // no more lines, break early
        };

        // Parse as many integers from this line as possible
        for val in line.split_whitespace() {
            let num: i32 = val.parse()?;
            nums.push(num);
            read_count += 1;
            if read_count == n {
                break 'outer;
            }
        }
    }

    // If we didn't get n integers, it's an error
    if read_count < n {
        return Err("Not enough integer values provided".into());
    }

    // Compute and print the result
    let result = count_partitions(&nums);
    println!("{}", result);

    Ok(())
}