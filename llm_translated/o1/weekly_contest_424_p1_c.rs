// Problem: Weekly Contest 424 Problem 1

use std::error::Error;
use std::io::{self, BufRead};

// Translated from the original C function "countValidSelections".
// Preserves the exact logic and returns the integer result.
fn count_valid_selections(nums: &[i32]) -> i32 {
    let mut sum_left = 0;
    let mut sum_right = 0;
    let mut ret = 0;

    // First pass: compute the total sum to initialize sum_right
    for &num in nums {
        sum_right += num;
    }

    // Second pass: adjust sums and count valid selections
    for &num in nums {
        if num != 0 {
            sum_left += num;
            sum_right -= num;
        } else {
            // If the current number is 0, check the conditions
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
    // Read the number of elements (n) from stdin
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let n: usize = line.trim().parse()?;

    // Read n integers from stdin
    // The input may be split across multiple lines
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        line.clear();
        if io::stdin().read_line(&mut line)? == 0 {
            // End of input
            break;
        }
        for num_str in line.split_whitespace() {
            let val: i32 = num_str.parse()?;
            nums.push(val);
            if nums.len() == n {
                break;
            }
        }
    }

    // Calculate the result using the translated function
    let result = count_valid_selections(&nums);

    // Print the result to stdout
    println!("{}", result);

    Ok(())
}