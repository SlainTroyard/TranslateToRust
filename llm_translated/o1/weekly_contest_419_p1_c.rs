// Weekly Contest 419 Problem 1 in Rust
// 
// This program reads from STDIN and writes to STDOUT with the exact same 
// input/output format as the original C code.
//
// It preserves the same logic for finding the sum of the top X most frequent (and
// highest valued in case of a tie) elements in every consecutive subarray of size k.
//
// Usage:
// 1) Provide "k x" on the first line, where k and x are integers.
// 2) Provide the integer "numsSize" on the next line.
// 3) Provide exactly numsSize integers (spread across lines or on the same line).
//    The code reads them in order.
// 4) The program outputs the transformed sums separated by space, followed by a newline.

use std::error::Error;
use std::io::{self, BufRead};

/// Finds, for each consecutive subarray of length k, the sum of the top x most
/// frequent elements (picking higher-valued elements first when frequencies tie).
fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let nums_size = nums.len();
    let num_results = nums_size.saturating_sub(k) + 1;
    let mut answer = Vec::with_capacity(num_results);

    for start in 0..num_results {
        // Frequency array for values in [0..50]
        let mut freq = [0; 51];
        // Count the frequency of each number in the current subarray
        for offset in 0..k {
            let idx = nums[start + offset] as usize;
            freq[idx] += 1;
        }

        // Collect (value, count) pairs
        let mut elements = Vec::new();
        for (val, &count) in freq.iter().enumerate() {
            if count > 0 {
                elements.push((val as i32, count));
            }
        }

        // Sort by count descending, then value descending
        elements.sort_by(|&(val_a, count_a), &(val_b, count_b)| {
            if count_a == count_b {
                val_b.cmp(&val_a)
            } else {
                count_b.cmp(&count_a)
            }
        });

        // Sum up to x "top" elements
        let mut sum = 0;
        let elements_to_sum = x.min(elements.len());
        for i in 0..elements_to_sum {
            let (value, count) = elements[i];
            sum += value * count as i32;
        }

        answer.push(sum);
    }

    answer
}

fn main() -> Result<(), Box<dyn Error>> {
    // We read lines from stdin in a way that matches the original code's input approach
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and x
    let first_line = lines.next().ok_or("Missing input for k and x")??;
    let mut parts = first_line.split_whitespace();
    let k: i32 = parts.next().ok_or("Missing input for k")?.parse()?;
    let x: i32 = parts.next().ok_or("Missing input for x")?.parse()?;

    // Read numsSize
    let second_line = lines.next().ok_or("Missing input for numsSize")??;
    let nums_size: usize = second_line.trim().parse()?;

    // Collect all remaining tokens and parse them as integers
    let mut tokens = Vec::new();
    for line in lines {
        for token in line?.split_whitespace() {
            tokens.push(token.to_string());
        }
    }
    if tokens.len() < nums_size {
        return Err("Not enough numbers provided for the array".into());
    }

    // Build the nums array
    let mut nums = Vec::with_capacity(nums_size);
    for i in 0..nums_size {
        nums.push(tokens[i].parse()?);
    }

    // Compute the result using the translated logic
    let answer = find_x_sum(&nums, k as usize, x as usize);

    // Print the answer array separated by space, followed by a newline
    for val in &answer {
        print!("{} ", val);
    }
    println!();

    Ok(())
}