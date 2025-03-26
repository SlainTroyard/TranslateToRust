use std::io::{self, BufRead};

// Function to get the maximum of two numbers
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// Main function to calculate the maximum frequency
fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0;               // Tracks the number of elements equal to k so far
    let mut f1 = [0; 51];          // Stores the maximum frequency for each number
    let mut max_f1 = 0;            // Maximum value in f1 array
    let mut f2 = 0;                // Specific calculation result

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

    let first_line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing first line"))??;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing n"))?.parse()?;
    let k: i32 = iter.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing k"))?.parse()?;

    // Read the array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing number in nums"))??;
        let num: i32 = line.parse()?;
        nums.push(num);
    }

    // Calculate the result
    let result = max_frequency(&nums, k);

    // Output the result
    println!("{}", result);

    Ok(())
}