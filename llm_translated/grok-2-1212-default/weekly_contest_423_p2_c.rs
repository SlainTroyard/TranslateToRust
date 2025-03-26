use std::io::{self, Read};

// Function to return the minimum of two integers
fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

// Function to return the maximum of three integers
fn max(a: i32, b: i32, c: i32) -> i32 {
    let d = if a > b { a } else { b };  // Find the maximum of a and b
    if d > c { d } else { c }            // Return the maximum of d and c
}

// Function to find the length of the longest increasing subarrays
fn max_increasing_subarrays(nums: &[i32]) -> i32 {
    let mut maxlen = 0;  // Variable to store the maximum length of subarrays
    let mut i = 1;       // Start from the second element
    let mut max1 = 1;    // Variable to store the length of the previous increasing subarray
    
    // Traverse the array
    while i < nums.len() {
        let mut max2 = 1;  // Variable to store the length of the current increasing subarray
        // Find the length of the current increasing subarray
        while i < nums.len() && nums[i] > nums[i - 1] {
            max2 += 1;
            i += 1;
        }
        let temp = min(max1, max2);  // Find the minimum length between the previous and current subarray
        maxlen = max(maxlen, temp, max2 / 2);  // Update the maximum length based on the calculated values
        max1 = max2;  // Update the length of the previous subarray
        i += 1;  // Move to the next element
    }
    maxlen  // Return the maximum length found
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Parse the input
    let mut lines = input.lines();
    let nums_size: usize = lines.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = lines.next().unwrap().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the function and display the result
    let result = max_increasing_subarrays(&nums);
    println!("{}", result);

    Ok(())
}