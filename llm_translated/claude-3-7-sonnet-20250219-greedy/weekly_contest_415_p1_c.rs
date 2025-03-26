// Problem: Weekly Contest 415 Problem 1
use std::io::{self, BufRead};

fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
    let mut result = vec![0; 2];
    let mut count = 0;
    
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                result[count] = nums[i];
                count += 1;
                break;
            }
        }
        if count == 2 {
            break;
        }
    }
    
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line and parse it as an integer
    let mut num_size: usize = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .trim()
        .parse()
        .expect("Failed to parse number");
    
    // Add 2 to num_size as in the original C code
    num_size += 2;
    
    // Read the numbers
    let mut nums = Vec::with_capacity(num_size);
    let numbers_line = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input");
    
    for num_str in numbers_line.split_whitespace().take(num_size) {
        let num: i32 = num_str.parse().expect("Failed to parse number");
        nums.push(num);
    }
    
    // Get the sneaky numbers
    let result = get_sneaky_numbers(&nums);
    
    // Print the result
    for i in 0..2 {
        print!("{} ", result[i]);
    }
}