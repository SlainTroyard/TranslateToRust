use std::io::{self, BufRead};

/// Count the number of partition positions i (where 0 <= i < nums.len()-1)
/// such that the difference between the sum of the first i+1 elements and the sum
/// of the rest is even.
///
/// This directly translates the logic from the provided C code.
fn count_partitions(nums: &[i32]) -> i32 {
    let mut count = 0;
    // Iterate over all valid partition indices.
    for i in 0..nums.len() - 1 {
        // Compute sum of elements from index 0 to i (inclusive).
        let left_sum: i32 = nums[..=i].iter().sum();
        // Compute sum of elements from index i+1 to end.
        let right_sum: i32 = nums[i + 1..].iter().sum();
        // If the difference (left_sum - right_sum) is even, count this partition.
        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    // Get a handle to standard input.
    let stdin = io::stdin();
    // Create a locked handle for buffered reading.
    let reader = stdin.lock();

    // Read all input lines and split by whitespace, then collect into a vector of strings.
    // This approach handles inputs that span multiple lines or have multiple values per line,
    // exactly like the original C code's scanf behavior.
    let tokens: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<String>>())
        .collect();

    // Ensure that we have at least one token for n.
    if tokens.is_empty() {
        eprintln!("No input provided.");
        return;
    }

    // Parse the first token as the size n.
    let n: usize = tokens[0].parse().expect("Error reading n");
    
    // Ensure that there are enough tokens for the n numbers.
    if tokens.len() < n + 1 {
        eprintln!("Not enough numbers provided in the input.");
        return;
    }

    // Parse the next n tokens into a vector of i32 values.
    let nums: Vec<i32> = tokens[1..=n]
        .iter()
        .map(|token| token.parse().expect("Error reading a number from input"))
        .collect();

    // Compute the result by counting the valid partitions.
    let result = count_partitions(&nums);

    // Output the result in the exact expected format.
    println!("{}", result);
}