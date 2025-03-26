use std::io::{self, BufRead};
use std::cmp;

/// Returns the maximum of two integers
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

/// Main function implementation
fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0;               // Track how many elements equal to k so far
    let mut f1 = [0; 51];         // Store the maximum frequency for each number
    let mut max_f1 = 0;           // Maximum value in the f1 array
    let mut f2 = 0;               // Specific calculation result
    
    for &x in nums {
        // Update f2 - consider the current element
        f2 = max(f2, max_f1) + if x == k { 1 } else { 0 };
        
        // Update the frequency for the current number
        f1[x as usize] = max(f1[x as usize], f0) + 1;
        
        // If the current element equals k, update f0
        if x == k {
            f0 += 1;
        }
        
        // Update the maximum frequency
        max_f1 = max(max_f1, f1[x as usize]);
    }
    
    // Return the final result
    max(max_f1, f2)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line: n and k
    let first_line = lines.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    
    let n: usize = parts.next()
        .expect("Missing n value")
        .parse()
        .expect("n must be an integer");
    
    let k: i32 = parts.next()
        .expect("Missing k value")
        .parse()
        .expect("k must be an integer");
    
    // Read the nums array
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().expect("Each number must be an integer"))
        .collect();
    
    // Check if we have the correct number of elements
    if nums.len() != n {
        eprintln!("Error: Expected {} numbers, got {}", n, nums.len());
        std::process::exit(1);
    }
    
    // Call the function to calculate the result
    let result = max_frequency(&nums, k);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}