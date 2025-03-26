use std::io::{self, BufRead};

/// Returns the first two numbers in `nums` that appear more than once.
/// This replicates the exact logic of the C function `getSneakyNumbers`.
fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
    let mut result = vec![0, 0];
    let mut count = 0;

    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                result[count] = nums[i];
                count += 1;
                break;
            }
        }
        if count == 2 {
            break;
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the first integer (numSize) from stdin
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let mut num_size: usize = line.trim().parse()?;

    // Increase numSize by 2, matching the C code logic
    num_size += 2;

    // Read 'num_size' integers from stdin, allowing multiple lines
    let mut nums = Vec::with_capacity(num_size);
    let mut read_count = 0;
    while read_count < num_size {
        line.clear();
        if io::stdin().read_line(&mut line)? == 0 {
            // If there's no more input to read, we stop
            // (same as the C code would stop receiving input)
            break;
        }
        for token in line.split_whitespace() {
            if read_count < num_size {
                nums.push(token.parse()?);
                read_count += 1;
            } else {
                break;
            }
        }
    }

    // Get the sneaky numbers
    let result = get_sneaky_numbers(&nums);

    // Print the result exactly like the C code: two numbers separated by a space
    for &r in &result {
        print!("{} ", r);
    }

    // Return Ok to indicate successful completion
    Ok(())
}