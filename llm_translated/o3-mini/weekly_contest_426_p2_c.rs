use std::io::{self, BufRead, Write};

/// This function implements the logic to find the largest outlier as described in the C code.
/// It takes in a slice of integers and returns the largest outlier, or -1001 if none exist.
fn get_largest_outlier(nums: &[i32]) -> i32 {
    // Calculate the total sum of the elements in the array.
    let total_sum: i32 = nums.iter().sum();
    
    // Create an array of frequency counts for each number in the range [-1000, 1000].
    // The index mapping is: num + 1000.
    let mut counts = [0; 2001];
    for &num in nums {
        // We are assured that the number is in the range [-1000, 1000]
        counts[(num + 1000) as usize] += 1;
    }
    
    // Initialize answer to -1001 as in the original code.
    let mut ans = -1001;
    
    // Iterate over the elements to determine if they are outliers based on the given criteria.
    for &x in nums {
        let curr_sum = total_sum - x;
        // Check if current sum is even, we can use bitwise & on i32 safely because of twos complement representation
        if curr_sum & 1 == 0 {
            let half = curr_sum / 2;
            let threshold = if half == x { 1 } else { 0 };
            
            // Verify that half is in the valid range [-1000, 1000]
            if half >= -1000 && half <= 1000 {
                if counts[(half + 1000) as usize] > threshold {
                    ans = ans.max(x);
                }
            }
        }
    }
    
    ans
}

/// This function reads integers from the standard input.
/// It returns a vector of i32 values parsed from the input.
fn read_input() -> io::Result<(usize, Vec<i32>)> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array.
    let nums_size = loop {
        if let Some(line) = lines.next() {
            let line = line?;
            if !line.trim().is_empty() {
                // Try to parse the first available number from the input
                let parsed = line.trim().parse::<usize>().map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidData, format!("Invalid number: {}", e))
                });
                break parsed?;
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No input provided"));
        }
    };

    // Read the next numbers until we have exactly nums_size numbers.
    let mut nums = Vec::with_capacity(nums_size);
    while nums.len() < nums_size {
        // Read a line from stdin.
        if let Some(line) = lines.next() {
            let line = line?;
            // Split on whitespace and try to parse each number.
            for token in line.split_whitespace() {
                if nums.len() < nums_size {
                    let num = token.parse::<i32>().map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid integer '{}': {}", token, e),
                        )
                    })?;
                    nums.push(num);
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    }

    // Ensure that we have read exactly nums_size numbers.
    if nums.len() != nums_size {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "The number of elements provided does not match the specified size",
        ));
    }

    Ok((nums_size, nums))
}

fn main() -> io::Result<()> {
    // Set up buffered standard output.
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    // Parse the input.
    let (_nums_size, nums) = read_input()?;

    // Compute the largest outlier (following the exact same logic as the original C code).
    let result = get_largest_outlier(&nums);

    // Print the result to standard output (exact same format as the C code).
    writeln!(out, "{}", result)?;

    Ok(())
}