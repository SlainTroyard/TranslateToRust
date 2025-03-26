// Problem: Weekly Contest 434 Problem 3
use std::io;
use std::cmp::max;

fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0;           // Tracks how many elements are equal to k
    let mut f1 = [0; 51];     // Stores the maximum frequency for each number
    let mut max_f1 = 0;       // Maximum value in the f1 array
    let mut f2 = 0;           // Specific calculation result

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

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read n and k from stdin");
    let mut parts = input.trim().split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Invalid value for n");
    let k: i32 = parts.next().unwrap().parse().expect("Invalid value for k");

    // Read the array elements
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read nums from stdin");
    let nums: Vec<i32> = input.trim()
                              .split_whitespace()
                              .map(|x| x.parse().expect("Invalid value in nums array"))
                              .collect();

    if nums.len() != n {
        eprintln!("Invalid number of elements: expected {}, got {}", n, nums.len());
        std::process::exit(1);
    }

    // Call the function and compute the result
    let result = max_frequency(&nums, k);

    // Output the result
    println!("{}", result);
}