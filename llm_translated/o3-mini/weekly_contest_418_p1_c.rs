use std::error::Error;
use std::io::{self, Read};

/// Computes the "max good number" following the logic of the C version.
///
/// The algorithm:
/// 1. For each of the first three numbers in `nums` (ignoring the remainder if any),
///    count how many times (from 0 to 6) the condition `((nums[i] << j) & 64) == 0` holds
///    consecutively. This count is stored in the `times` array.
/// 2. A helper `store` array holding indices [0, 1, 2] is sorted using a custom comparison:
///    For a pair of indices (store[j], store[j+1]), compute two values:
///      - value1 = (nums[store[j]] << (7 - times[store[j+1]])) + nums[store[j+1]]
///      - value2 = (nums[store[j+1]] << (7 - times[store[j]])) + nums[store[j]]
///    If value2 >= value1 then swap the two indices.
/// 3. Finally, the result is computed using these sorted indices.
///
/// Note:
///   - The C code processes only the first three numbers.
///   - Input/Output formatting is exactly preserved.
fn max_good_number(nums: &[i32]) -> i32 {
    // Count consecutive shifts that result in a zero in the 7th bit for each number.
    // Only consider the first three numbers.
    let mut times = [0; 3];
    for (i, &num) in nums.iter().enumerate().take(3) {
        for j in 0..7 {
            // (num << j) & 64 checks whether the bit in position 6 (0-indexed) is 0.
            if (num << j) & 64 == 0 {
                times[i] += 1;
            } else {
                break;
            }
        }
    }

    // Create a store array with indices 0, 1, 2. This is used for custom ordering.
    let mut store = [0, 1, 2];
    // Perform two full passes of adjacent swaps to order the indices.
    for _ in 1..=2 {
        for j in 0..2 {
            let idx_a = store[j];
            let idx_b = store[j + 1];
            // Calculate the two potential values using the shifting logic.
            let value1 = (nums[idx_a] << (7 - times[idx_b])) + nums[idx_b];
            let value2 = (nums[idx_b] << (7 - times[idx_a])) + nums[idx_a];
            // If value2 is larger or equal, swap the indices.
            if value2 >= value1 {
                store.swap(j, j + 1);
            }
        }
    }

    // Compute the final result using the sorted indices.
    (nums[store[0]] << (14 - times[store[1]] - times[store[2]])) +
        (nums[store[1]] << (7 - times[store[2]])) +
        nums[store[2]]
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read entire standard input into a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Split the input by whitespace and create an iterator.
    let mut tokens = input.split_whitespace();

    // Read the size of the array (number of integers to read).
    let num_size: usize = tokens
        .next()
        .ok_or("Expected an integer for array size")?
        .parse()?;
    
    // Read the numbers into a vector.
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let num = tokens
            .next()
            .ok_or("Expected more numbers in input")?
            .parse()?;
        nums.push(num);
    }

    // Compute the result using the provided algorithm.
    let result = max_good_number(&nums);

    // Print the result following the exact output format.
    println!("{}", result);
    Ok(())
}