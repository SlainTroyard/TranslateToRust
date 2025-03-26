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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line and parse as an integer
    let mut num_size: usize = lines.next()
        .expect("Failed to read line")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse number");
    
    // Adjust num_size as in the original C code
    num_size += 2;
    
    // Read the second line with the numbers
    let nums: Vec<i32> = lines.next()
        .expect("Failed to read line")
        .expect("Failed to read line")
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse number"))
        .collect();
    
    // Call the function
    let result = get_sneaky_numbers(&nums[0..num_size]);
    
    // Print the result with spaces
    for (i, &num) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", num);
    }
    
    Ok(())
}