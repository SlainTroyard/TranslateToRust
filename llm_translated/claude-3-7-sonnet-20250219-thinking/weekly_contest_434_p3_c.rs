use std::io::{self, Read};
use std::cmp::max;

// Function to find the maximum frequency
fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0;               // Tracks how many elements equal to k so far
    let mut f1 = [0; 51];         // Stores maximum frequency for each number
    let mut max_f1 = 0;           // Maximum value in the f1 array
    let mut f2 = 0;               // Specific calculation result
    
    for &x in nums {
        // Update f2 - consider current element
        f2 = max(f2, max_f1) + if x == k { 1 } else { 0 };
        
        // Update frequency of current number
        f1[x as usize] = max(f1[x as usize], f0) + 1;
        
        // If current element equals k, update f0
        f0 += if x == k { 1 } else { 0 };
        
        // Update maximum frequency
        max_f1 = max(max_f1, f1[x as usize]);
    }
    
    // Return final result
    max(max_f1, f2)
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    
    // Parse n and k
    let n: usize = match tokens.next() {
        Some(val) => match val.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Error reading input for n");
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Error reading input for n");
            std::process::exit(1);
        }
    };
    
    let k: i32 = match tokens.next() {
        Some(val) => match val.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Error reading input for k");
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Error reading input for k");
            std::process::exit(1);
        }
    };
    
    // Allocate vector and read the array
    let mut nums = Vec::with_capacity(n);
    for i in 0..n {
        let num: i32 = match tokens.next() {
            Some(val) => match val.parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Error reading input for nums[{}]", i);
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!("Error reading input for nums[{}]", i);
                std::process::exit(1);
            }
        };
        nums.push(num);
    }
    
    // Call function to calculate result
    let result = max_frequency(&nums, k);
    
    // Output result
    println!("{}", result);
}