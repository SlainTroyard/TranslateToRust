// LeetCode Weekly Contest 434 Problem 3 (Translation from C to Rust)
// Requirements:
// 1. Translate the ENTIRE file as a complete program, including main.
// 2. Preserve the EXACT logic of the original C code.
// 3. Use idiomatic Rust with proper error handling.
// 4. Maintain the EXACT SAME stdin/stdout format as the original code.
// 5. Add helpful comments where needed.

use std::io::{self, BufRead};
use std::process;

// A helper function that replicates the behavior of the C "max" function
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

// The core function that replicates the maxFrequency logic from the C code
fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0; // Tracks how many elements equal to k so far
    let mut f1 = [0; 51]; // Saves the max frequency for each number (0..50)
    let mut max_f1 = 0; // The maximum value in f1
    let mut f2 = 0; // A special calculation result

    // Process each element as in the C code
    for &x in nums {
        // Update f2 based on the current element
        f2 = max(f2, max_f1) + if x == k { 1 } else { 0 };

        // Update the frequency for the current number
        // (assuming x is within [0..50] as in the original code)
        let x_usize = x as usize;
        f1[x_usize] = max(f1[x_usize], f0) + 1;

        // If the current element equals k, increase f0
        if x == k {
            f0 += 1;
        }

        // Update the maximum frequency
        max_f1 = max(max_f1, f1[x_usize]);
    }

    // Return the maximum between max_f1 and f2
    max(max_f1, f2)
}

fn main() {
    // Prepare to read from stdin
    let stdin = io::stdin();

    // Read the first line, which should contain n and k
    let mut line = String::new();
    if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
        eprintln!("Error reading input for n and k");
        process::exit(1);
    }

    // Split the line into tokens for n and k
    let mut tokens = line.split_whitespace();
    let n_str = tokens.next();
    let k_str = tokens.next();

    // Error if we don't have two valid tokens
    if n_str.is_none() || k_str.is_none() {
        eprintln!("Error reading input for n and k");
        process::exit(1);
    }

    // Parse n
    let n: usize = match n_str.unwrap().parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error reading input for n and k");
            process::exit(1);
        }
    };

    // Parse k
    let k_val: i32 = match k_str.unwrap().parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error reading input for n and k");
            process::exit(1);
        }
    };

    // In C, we would call malloc here and check if it's NULL. In Rust, we normally
    // just create a vector. This is safe and won't usually fail in typical scenarios.
    // We'll still keep the structure consistent with the original code.
    let mut nums = Vec::with_capacity(n);

    // Now we need to read n integers in total, one by one, exactly as C scanf would.
    // If at any point we fail to read an integer, we print an error and exit.
    let mut count = 0;
    // We'll reuse the current line's leftover tokens if any remain.
    while count < n {
        if let Some(next_token) = tokens.next() {
            // We found a token in the current line. Attempt to parse it.
            match next_token.parse::<i32>() {
                Ok(val) => {
                    nums.push(val);
                    count += 1;
                }
                Err(_) => {
                    eprintln!("Error reading input for nums[{}]", count);
                    process::exit(1);
                }
            }
        } else {
            // No more tokens in the current line, read a new line
            line.clear();
            if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
                // We failed to read another integer entirely
                eprintln!("Error reading input for nums[{}]", count);
                process::exit(1);
            }
            tokens = line.split_whitespace();
        }
    }

    // Call the logic function
    let result = max_frequency(&nums, k_val);

    // Print the result, just like the C code does
    println!("{}", result);
}