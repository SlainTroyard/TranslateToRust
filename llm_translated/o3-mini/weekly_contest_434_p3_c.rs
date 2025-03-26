use std::cmp;
use std::io::{self, Read, Write};

/// Computes the result based on the algorithm described.
/// This function maintains tracking variables f0, f1, f2, and max_f1,
/// and processes each element in nums according to the logic described.
///
/// # Arguments
/// - `nums`: A slice of i32 values representing the input numbers.
/// - `k`: The special value used in the calculation.
///
/// # Returns
/// The final computed result as an i32.
fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0;               // Tracks how many times k has occurred so far.
    let mut f1 = [0; 51];         // Stores the maximum frequency for each possible digit index (0 to 50).
    let mut max_f1 = 0;           // The maximum value in the f1 array.
    let mut f2 = 0;               // Specific computed result.

    // Process each element in the nums slice.
    for &x in nums {
        // Update f2: set f2 = max(f2, max_f1) + (if x equals k then 1, else 0)
        f2 = cmp::max(f2, max_f1) + if x == k { 1 } else { 0 };

        // Check that x is within the valid range for f1 array indices.
        if x < 0 || (x as usize) >= f1.len() {
            writeln!(io::stderr(), "Value out of expected range (0..50): {}", x).unwrap();
            std::process::exit(1);
        }

        // Update the frequency for the current number:
        // f1[x] = max(f1[x], f0) + 1
        f1[x as usize] = cmp::max(f1[x as usize], f0) + 1;

        // If the element equals k, update f0.
        if x == k {
            f0 += 1;
        }

        // Update the maximum frequency seen so far.
        max_f1 = cmp::max(max_f1, f1[x as usize]);
    }

    // Return the maximum of max_f1 and f2.
    cmp::max(max_f1, f2)
}

fn main() {
    // Read the entire input from stdin.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        writeln!(io::stderr(), "Error reading input").unwrap();
        std::process::exit(1);
    }

    // Split the input by whitespace.
    let mut tokens = input.split_whitespace();

    // Parse n and k from the input.
    let n: usize = match tokens.next() {
        Some(token) => token.parse().unwrap_or_else(|_| {
            writeln!(io::stderr(), "Error reading input for n").unwrap();
            std::process::exit(1);
        }),
        None => {
            writeln!(io::stderr(), "Error reading input for n and k").unwrap();
            std::process::exit(1);
        }
    };

    let k: i32 = match tokens.next() {
        Some(token) => token.parse().unwrap_or_else(|_| {
            writeln!(io::stderr(), "Error reading input for k").unwrap();
            std::process::exit(1);
        }),
        None => {
            writeln!(io::stderr(), "Error reading input for n and k").unwrap();
            std::process::exit(1);
        }
    };

    // Parse the next n integers into a vector.
    let mut nums = Vec::with_capacity(n);
    for i in 0..n {
        match tokens.next() {
            Some(token) => {
                let num = token.parse::<i32>().unwrap_or_else(|_| {
                    writeln!(io::stderr(), "Error reading input for nums[{}]", i).unwrap();
                    std::process::exit(1);
                });
                nums.push(num);
            }
            None => {
                writeln!(io::stderr(), "Error reading input for nums[{}]", i).unwrap();
                std::process::exit(1);
            }
        }
    }

    // Call the function to calculate the result.
    let result = max_frequency(&nums, k);

    // Output the result to stdout.
    println!("{}", result);
}