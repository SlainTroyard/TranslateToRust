use std::error::Error;
use std::io::{self, Read};

/// Checks if there exists two consecutive subarrays of length `k` that are increasing.
/// This function preserves the algorithm logic of the original C code.
///
/// # Arguments
/// * `nums` - Slice of integers representing the array.
/// * `k` - The length of each subarray to check.
///
/// # Returns
/// * `true` if there is at least one valid increasing subarray pair, `false` otherwise.
fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
    // When subarray length is 1, each subarray is trivially increasing.
    if k == 1 {
        return true;
    }

    let n = nums.len();
    // Calculate the limit for the starting index `i` of the first subarray.
    // This follows the condition: i < numsSize - 2*k + 1.
    // If 2*k is greater than the array size, the loop simply won't run.
    let limit = if 2 * k > n { 0 } else { n - 2 * k + 1 };

    // Outer loop is executed twice as in the original C code.
    for _ in 0..2 {
        // Iterate over all possible starting indices `i` for the first subarray.
        for i in 0..limit {
            let s = i + k; // s is the starting index of the second subarray.
            let mut a = 0;
            // Check each consecutive pair in both subarrays.
            for z in 0..(k - 1) {
                // If both pairs are strictly increasing, increment `a`.
                if nums[i + z] < nums[i + z + 1] && nums[s + z] < nums[s + z + 1] {
                    a += 1;
                }
            }
            // If all comparisons (k-1 comparisons) succeeded, we found a valid pair.
            if a == k - 1 {
                return true;
            }
        }
    }
    // No valid increasing subarray pair was found.
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input from standard input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split input into tokens separated by whitespace.
    let mut tokens = input.split_whitespace();

    // Expect the first token to be the size of the array `n`.
    let n: usize = tokens
        .next()
        .ok_or("Expected array length 'n' as the first input.")?
        .parse()?;
    
    // Read `n` integers into a vector.
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = tokens
            .next()
            .ok_or("Expected an integer for the array.")?
            .parse()?;
        nums.push(num);
    }

    // Read the subarray length `k`.
    let k: usize = tokens
        .next()
        .ok_or("Expected subarray length 'k' after the array input.")?
        .parse()?;

    // Call the function to check for increasing subarrays.
    // Output must match the exact format of the original code.
    if has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}