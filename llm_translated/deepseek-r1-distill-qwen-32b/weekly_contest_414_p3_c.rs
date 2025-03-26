// Problem: Weekly Contest 414 Problem 3
// This is a direct translation of the C code into idiomatic Rust.

use std::io;

fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans: i64 = 0;
    let mut l: usize = 0;
    let len = nums.len();
    
    for r in 1..len {
        if nums[l] < nums[r] {
            ans += (r as i64 - l as i64) * nums[l] as i64;
            l = r;
        }
    }
    
    // Add the last segment contribution
    ans += (len as i64 - l as i64 - 1) * nums[l] as i64;
    ans
}

fn main() {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .unwrap_or_else(|e| {
            eprintln!("Error reading input: {}", e);
            std::process::exit(1);
        });
    
    let nums_size: usize = input.trim().parse()
        .unwrap_or_else(|e| {
            eprintln!("Invalid input for size: {}", e);
            std::process::exit(1);
        });
    
    // Read the array elements
    let mut nums = Vec::with_capacity(nums_size);
    input = String::new();
    io::stdin().read_line(&mut input)
        .unwrap_or_else(|e| {
            eprintln!("Error reading array: {}", e);
            std::process::exit(1);
        });
    
    let nums_slice: Vec<i32> = input.trim().split(' ')
        .map(|s| s.parse().unwrap_or_else(|e| {
            eprintln!("Invalid number: {}", e);
            std::process::exit(1);
        }))
        .collect();
    
    if nums_slice.len() != nums_size {
        eprintln!("Number of elements does not match the specified size.");
        std::process::exit(1);
    }
    
    nums = nums_slice;
    
    // Compute and print the result
    let result = find_maximum_score(&nums);
    println!("{}", result);
}