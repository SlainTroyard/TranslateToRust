use std::io::{self, BufRead};

// Function to get the maximum of two numbers
fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

// Main function to calculate the maximum frequency
fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0;               // Tracks the number of elements equal to k so far
    let mut f1 = [0; 51];         // Stores the maximum frequency of each number
    let mut max_f1 = 0;           // Maximum value in f1 array
    let mut f2 = 0;               // Specific calculation result

    for &x in nums {
        // Update f2 - consider the current element
        f2 = max(f2, max_f1) + if x == k { 1 } else { 0 };

        // Update the frequency of the current number
        f1[x as usize] = max(f1[x as usize], f0) + 1;

        // If the current element equals k, update f0
        f0 += if x == k { 1 } else { 0 };

        // Update the maximum frequency
        max_f1 = max(max_f1, f1[x as usize]);
    }

    // Return the final result
    max(max_f1, f2)
}

fn main() -> io::Result<()> {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse n and k
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // Read the array
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate the result
    let result = max_frequency(&nums, k);

    // Output the result
    println!("{}", result);

    Ok(())
}