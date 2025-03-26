use std::io;

/// Computes the maximum number of distinct elements achievable by adjusting each element within ±diff.
///
/// # Arguments
/// * `arr` - A mutable slice of integers to process.
/// * `diff` - The maximum allowed adjustment for each element.
///
/// # Returns
/// The maximum count of distinct elements after adjustments.
fn max_distinct_elements(arr: &mut [i32], diff: i32) -> i32 {
    // Sort the array to process elements in ascending order.
    arr.sort_unstable();
    let mut prev = i32::MIN; // Tracks the previous adjusted value (starts as -infinity)
    let mut distinct_count = 0;

    for &x in arr.iter() {
        // Calculate the lower bound for the current element's adjustment
        let lower = x - diff;
        // The candidate value is the next possible value after the previous adjustment
        let candidate = prev + 1;
        // Choose the maximum between the candidate and the lower bound
        let current_x = if candidate > lower { candidate } else { lower };

        // Check if the chosen value is within the allowed range
        if current_x <= x + diff {
            distinct_count += 1;
            prev = current_x; // Update previous for next iteration
        }
    }

    distinct_count
}

fn main() {
    let mut input = String::new();
    // Read all input into a single string
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");

    // Split input into tokens and parse integers
    let mut tokens = input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid integer"));

    // Extract n and diff from the first two tokens
    let n = tokens.next().expect("No n provided");
    let diff = tokens.next().expect("No diff provided");

    // Collect the next 'n' tokens as the array
    let mut arr: Vec<i32> = tokens.take(n as usize).collect();

    // Ensure we have exactly 'n' elements (matches original C's behavior)
    if arr.len() != n as usize {
        panic!("Not enough elements in array");
    }

    // Compute and print the result
    let result = max_distinct_elements(&mut arr, diff);
    println!("{}", result);
}