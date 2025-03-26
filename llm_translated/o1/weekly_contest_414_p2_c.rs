use std::cmp::max;
use std::io::{self, BufRead};

/// Calculates the maximum possible score based on the sorted `start` array and distance `d`.
/// This function reproduces the same logic used in the C code's `maxPossibleScore`.
fn max_possible_score(start: &mut [i32], d: i32) -> i32 {
    // Sort the array (idiomatic Rust instead of bubble sort)
    start.sort();

    let n = start.len();

    // Set up binary search boundaries
    // left and right correspond to the potential "score" increments
    let mut left: i64 = 0;
    let mut right: i64 = ((start[n - 1] as i64 + d as i64) - start[0] as i64) / (n as i64 - 1) + 1;

    // Binary search for the maximum mid value that satisfies the constraint
    while left + 1 < right {
        let mid = left + (right - left) / 2;

        // Check feasibility using the "check function" from the C code
        // sum tracks the last assigned "score" position
        let mut sum = i64::MIN;
        let mut valid = true;

        for &val in start.iter() {
            // sum + mid vs val -> whichever is larger is the new position
            let incremented = sum + mid;
            sum = max(incremented, val as i64);

            // If we exceed the maximum allowed position (val + d), it's invalid
            if sum > val as i64 + d as i64 {
                valid = false;
                break;
            }
        }

        if valid {
            left = mid;   // mid can be a valid score increment, so move left up
        } else {
            right = mid;  // mid is too large, so lower the upper bound
        }
    }

    left as i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the first line for startSize and d
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut parts = buffer.split_whitespace();

    // Parse startSize and d
    let start_size: usize = parts
        .next()
        .ok_or("Missing 'startSize' input")?
        .parse()?;
    let d: i32 = parts.next().ok_or("Missing 'd' input")?.parse()?;

    // Read enough values to fill `start_size` integers
    let mut start = Vec::with_capacity(start_size);
    let mut count = 0;
    while count < start_size {
        let mut line = String::new();
        // Read next line (could have multiple integers)
        if io::stdin().read_line(&mut line)? == 0 {
            // End of input, but we haven't read enough integers
            return Err("Not enough integers for 'start' array".into());
        }
        for num_str in line.split_whitespace() {
            let num: i32 = num_str.parse()?;
            start.push(num);
            count += 1;
            if count == start_size {
                break;
            }
        }
    }

    // Compute the result using the same logic as the C code
    let result = max_possible_score(&mut start, d);

    // Print the result (same output format as the C code: %d\n)
    println!("{}", result);

    Ok(())
}