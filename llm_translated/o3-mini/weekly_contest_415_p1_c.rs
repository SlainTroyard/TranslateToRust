use std::io::{self, BufRead, Write};

/// This function mimics the C function getSneakyNumbers:
/// It searches for the first two numbers in the slice that have a duplicate later in the slice.
///
/// # Parameters
/// - `nums`: A slice of integers.
///
/// # Returns
/// An array of 2 integers, corresponding to the "sneaky" duplicate numbers found.
///
/// # Panics
/// This function assumes that there will be at least two such numbers, as in the original C code.
fn get_sneaky_numbers(nums: &[i32]) -> [i32; 2] {
    let mut result = [0; 2];
    let mut count = 0;
    // Iterate over each element and look for a duplicate later in the slice.
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                result[count] = nums[i];
                count += 1;
                break; // Break out of inner loop once a duplicate is found.
            }
        }
        if count == 2 {
            break; // Stop when two sneaky numbers have been found.
        }
    }
    result
}

fn main() -> io::Result<()> {
    // Use stdin locking for efficient reading.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read first non-empty line to get the size.
    let first_line = loop {
        if let Some(line) = lines.next() {
            let line = line?;
            if !line.trim().is_empty() {
                break line;
            }
        } else {
            // If input ends unexpectedly, exit.
            return Ok(());
        }
    };

    // Parse the first number: this corresponds to numSize in C code.
    let parsed_size: usize = first_line.trim().parse().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("Failed to parse first number: {}", e))
    })?;
    // Add 2, as in the C code: numSize += 2;
    let total_numbers = parsed_size + 2;

    // Prepare a vector to collect the numbers.
    let mut nums: Vec<i32> = Vec::with_capacity(total_numbers);

    // We already used first_line, so now we need to gather total_numbers integers from input.
    // We can use split_whitespace across remaining lines.
    // But we must include the possibility that the first line might have extra tokens.
    {
        // First, process tokens from the first line after the first integer.
        let mut tokens = first_line.split_whitespace();
        // Skip the first token (already used as parsed_size).
        tokens.next();
        for token in tokens {
            if nums.len() >= total_numbers {
                break;
            }
            let number: i32 = token.parse().map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidInput, format!("Failed to parse number: {}", e))
            })?;
            nums.push(number);
        }
    }

    // Process the rest of the lines until we have total_numbers integers.
    while nums.len() < total_numbers {
        if let Some(line) = lines.next() {
            for token in line?.split_whitespace() {
                if nums.len() >= total_numbers {
                    break;
                }
                let number: i32 = token.parse().map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidInput, format!("Failed to parse number: {}", e))
                })?;
                nums.push(number);
            }
        } else {
            break;
        }
    }

    // If we haven't gathered enough numbers, it's an input error.
    if nums.len() < total_numbers {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Not enough numbers provided in input.",
        ));
    }

    // Call the function to get the two sneaky numbers.
    let result = get_sneaky_numbers(&nums);

    // Print the results exactly as in the C code, separated by a space.
    // Note: The original C code prints a trailing space after each number.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for &num in result.iter() {
        write!(handle, "{} ", num)?;
    }
    // Flush stdout to ensure output is printed.
    handle.flush()?;

    Ok(())
}