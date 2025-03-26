use std::io::{self, Read};

/// Calculates the maximum score as described in LeetCode Weekly Contest 414 Problem 3
///
/// This function processes the array by finding positions where nums[l] < nums[r],
/// and accumulating scores based on these positions.
fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans = 0i64;
    let mut l = 0;
    let mut r = 1;
    
    while r < nums.len() {
        if nums[l] < nums[r] {
            ans += (r - l) as i64 * nums[l] as i64;
            l = r;
        }
        r += 1;
    }
    
    // Final contribution
    ans + (r - l - 1) as i64 * nums[l] as i64
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let nums_size: usize = input.trim().parse()?;
    
    // Allocate vector for the numbers
    let mut nums = Vec::with_capacity(nums_size);
    
    // Read each number individually
    for _ in 0..nums_size {
        let mut num_input = String::new();
        io::stdin().read_line(&mut num_input)?;
        nums.push(num_input.trim().parse::<i32>()?);
    }
    
    // Calculate and print the result
    println!("{}", find_maximum_score(&nums));
    
    Ok(())
}