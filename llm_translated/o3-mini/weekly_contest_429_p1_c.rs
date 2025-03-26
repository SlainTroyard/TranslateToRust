use std::io::{self, BufRead};

/// Function to compute the minimum operations as per the problem statement.
/// It iterates from the end of the array, counts occurrences of each number (0-100),
/// and if a duplicate is found, returns (i + 3) / 3, where i is the index of that duplicate.
fn minimum_operations(nums: &[i32]) -> i32 {
    // Fixed-size array for counting occurrences of numbers 0 through 100.
    let mut count = [0; 101];

    // Iterate from the last index down to 0.
    // The method `enumerate` is avoided in this reversed loop to directly use indices.
    for i in (0..nums.len()).rev() {
        // Increase the count of the current number.
        let num = nums[i] as usize;
        count[num] += 1;
        // If we have seen this number before (duplicate found)
        if count[num] > 1 {
            // (i + 3) / 3 is used for the answer, with integer division
            return ((i + 3) / 3) as i32;
        }
    }
    // If no duplicate found, return 0.
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the standard input handle using a buffered reader.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    // Read the first integer n from input.
    reader.read_line(&mut input)?;
    let n: usize = input.trim().parse()?;
    input.clear();

    // Prepare a vector to hold the n numbers.
    // The C code expects exactly n numbers, and these may be spread across lines.
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    while nums.len() < n {
        // Read a line and split it by whitespace.
        reader.read_line(&mut input)?;
        for token in input.split_whitespace() {
            // Stop reading if we've collected all numbers.
            if nums.len() == n {
                break;
            }
            let num = token.trim().parse()?;
            nums.push(num);
        }
        input.clear();
    }

    // Compute the result.
    let result = minimum_operations(&nums);

    // Print the result exactly as per the original C code.
    println!("{}", result);
    Ok(())
}