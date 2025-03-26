use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Lock stdin for efficient reading
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line, which contains the first token (n) and possibly some numbers.
    let first_line = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Expected input"))??;
    let mut tokens = first_line.split_whitespace();

    // Parse the number of elements, n.
    let n_str = tokens
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Expected n"))?;
    let n: usize = n_str
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    // Initialize the vector for numbers with capacity n.
    let mut nums = Vec::with_capacity(n);

    // Process any numbers provided on the first line after n.
    for token in tokens {
        if nums.len() < n {
            let num = token
                .parse::<i32>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
            nums.push(num);
        }
    }

    // Continue reading lines until we've read exactly n numbers.
    while nums.len() < n {
        let line = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Not enough numbers provided"))??;
        for token in line.split_whitespace() {
            if nums.len() < n {
                let num = token
                    .parse::<i32>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
                nums.push(num);
            }
        }
    }

    // Compute the result using the same algorithm logic as the C++ code.
    let result = count_partitions(&nums);

    // Print the result with the exact same output format.
    println!("{}", result);
    Ok(())
}

// This function implements the logic:
// If the sum of the array is odd, return 0.
// If the sum is even, return nums.len() - 1.
fn count_partitions(nums: &[i32]) -> i32 {
    let total_sum: i32 = nums.iter().sum();
    if total_sum % 2 != 0 {
        0
    } else {
        (nums.len() - 1) as i32
    }
}