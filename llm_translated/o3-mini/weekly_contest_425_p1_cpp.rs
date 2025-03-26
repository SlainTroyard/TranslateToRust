use std::io::{self, BufRead};

// Function to compute the minimum sum subarray as per the given constraints.
// It iterates over all subarrays and checks if the subarray length is within [l, r]
// and if its sum is greater than 0. It returns -1 if no such subarray exists.
fn minimum_sum_subarray(nums: &[i32], l: usize, r: usize) -> i32 {
    let n = nums.len();
    // Initialize mini with the maximum possible value (similar to INT_MAX in C++)
    let mut mini = i32::MAX;
    for i in 0..n {
        let mut curr_sum = 0;
        for j in i..n {
            curr_sum += nums[j];
            let length = j - i + 1;
            if length >= l && length <= r && curr_sum > 0 {
                mini = mini.min(curr_sum);
            }
        }
    }
    // If mini remains unchanged, return -1 as required
    if mini == i32::MAX { -1 } else { mini }
}

fn main() -> io::Result<()> {
    // Create a buffered reader for standard input
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    // Read the size of the array (n) from stdin
    input.clear();
    reader.read_line(&mut input)?;
    let n: usize = input.trim().parse().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid input for n: {}", e))
    })?;

    // Read the array elements (nums)
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    let mut count = 0;
    // Continue reading until we've read exactly n numbers
    while count < n {
        input.clear();
        reader.read_line(&mut input)?;
        for token in input.split_whitespace() {
            if count < n {
                let num: i32 = token.parse().map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid number: {}", e))
                })?;
                nums.push(num);
                count += 1;
            } else {
                break;
            }
        }
    }

    // Read the range [l, r]
    let mut lr: Vec<i32> = Vec::with_capacity(2);
    while lr.len() < 2 {
        input.clear();
        reader.read_line(&mut input)?;
        for token in input.split_whitespace() {
            let val: i32 = token.parse().map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid range value: {}", e))
            })?;
            lr.push(val);
            if lr.len() == 2 {
                break;
            }
        }
    }
    // Convert l and r to usize, as lengths are non-negative
    let l = lr[0] as usize;
    let r = lr[1] as usize;

    // Compute the minimum sum subarray using the helper function
    let result = minimum_sum_subarray(&nums, l, r);

    // Output the result (exactly as the original C++ code)
    println!("{}", result);

    Ok(())
}