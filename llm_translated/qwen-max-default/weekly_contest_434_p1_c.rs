use std::io::{self, BufRead};

/// Counts the number of valid partitions in the array `nums` where the difference
/// between the sum of the left and right parts is even.
fn count_partitions(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut count = 0;

    for i in 0..nums_size - 1 {
        let (left_sum, right_sum) = nums.split_at(i + 1);
        let left_sum: i32 = left_sum.iter().sum();
        let right_sum: i32 = right_sum.iter().sum();

        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }
    }

    count
}

fn main() -> io::Result<()> {
    // Read the number of elements from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Failed to read input"))??
        .parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Failed to parse input"))?;

    // Read the array of numbers from stdin
    let nums: Vec<i32> = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Failed to read input"))??
        .split_whitespace()
        .map(|x| x.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Failed to parse input")))
        .collect::<Result<_, _>>()?;

    // Ensure the number of elements matches the input size
    if nums.len() != n {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Input size does not match the number of elements"));
    }

    // Calculate the number of valid partitions
    let result = count_partitions(&nums);

    // Print the result to stdout
    println!("{}", result);

    Ok(())
}