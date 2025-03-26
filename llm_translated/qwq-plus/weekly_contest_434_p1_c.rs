use std::io;

/// Counts the number of partitions in the array where the difference between left and right sums is even.
///
/// # Arguments
///
/// * `nums` - A slice of integers representing the array.
///
/// # Returns
///
/// The count of valid partitions.
fn count_partitions(nums: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() - 1 {
        // Calculate left sum from start to current index
        let left_sum: i32 = nums[0..=i].iter().sum();
        // Calculate right sum from next index to end
        let right_sum: i32 = nums[i + 1..].iter().sum();
        // Check if the difference is even
        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut input = String::new();
    // Read all input, panics on error (mirroring C's undefined behavior on failure)
    io::stdin().read_to_string(&mut input).unwrap();
    // Parse input into integers
    let mut iter = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    // Read first value as n (number of elements)
    let n = iter.next().unwrap();
    // Collect next `n` values into a vector
    let nums: Vec<i32> = iter.take(n as usize).collect();
    // Output the result as per problem's format
    println!("{}", count_partitions(&nums));
}