use std::error::Error;
use std::io::{self, BufRead};

// This function performs the same logic as countValidSelections in C.
// It iterates over the numbers, updating sum_left and sum_right, and adds to ret accordingly.
fn count_valid_selections(nums: &[i32]) -> i32 {
    let mut sum_left = 0;
    // Calculate the initial sum_right as the sum of all elements
    let mut sum_right: i32 = nums.iter().sum();
    let mut ret = 0;

    // Iterate over each element in nums
    for &num in nums {
        if num != 0 {
            // For non-zero, update sum_left and sum_right
            sum_left += num;
            sum_right -= num;
        } else {
            // For zero, check the condition and update ret as per the rules
            if sum_left == sum_right {
                ret += 2;
            } else if (sum_left - sum_right).abs() == 1 {
                ret += 1;
            }
        }
    }
    ret
}

fn main() -> Result<(), Box<dyn Error>> {
    // Using a buffered reader for efficient input handling.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first non-empty line containing the integer n.
    let first_line = loop {
        if let Some(line) = lines.next() {
            let line = line?;
            if !line.trim().is_empty() {
                break line;
            }
        } else {
            return Err("Missing input for number of elements".into());
        }
    };

    // Parse the first integer from the first line.
    // The original C code uses scanf("%d", &n), so we do the same.
    let n: usize = first_line
        .trim()
        .split_whitespace()
        .next()
        .ok_or("Failed to find the number of elements")?
        .parse()?;

    // Read the next n integers from the input. The numbers may be distributed across multiple lines.
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        // Get the next line; return error if input ends unexpectedly.
        let line = match lines.next() {
            Some(l) => l?,
            None => break,
        };
        for token in line.split_whitespace() {
            if nums.len() >= n {
                break;
            }
            let number: i32 = token.parse()?;
            nums.push(number);
        }
    }

    // If the number of collected numbers is less than n, it's an error.
    if nums.len() < n {
        return Err("Not enough numbers provided".into());
    }

    // Apply the algorithm to count valid selections.
    let result = count_valid_selections(&nums);

    // Print the result following the original stdout formatting.
    println!("{}", result);

    Ok(())
}