use std::io::{self, BufRead};

// Function to count valid selections based on the problem logic
fn count_valid_selections(nums: &[i32]) -> i32 {
    let mut sum_left = 0;
    let mut sum_right: i32 = nums.iter().sum();
    let mut ret = 0;

    for &num in nums {
        if num != 0 {
            sum_left += num;
            sum_right -= num;
        } else {
            if sum_left == sum_right {
                ret += 2;
            } else if (sum_left - sum_right).abs() == 1 {
                ret += 1;
            }
        }
    }
    ret
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reading input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line to get the size of the nums array
    let n: usize = lines
        .next()
        .ok_or("Missing input")??
        .trim()
        .parse()
        .map_err(|e| format!("Failed to parse size: {}", e))?;

    // Parse the second line to get the nums array
    let nums: Vec<i32> = lines
        .next()
        .ok_or("Missing nums input")??
        .trim()
        .split_whitespace()
        .map(|num| num.parse().map_err(|e| format!("Failed to parse number: {}", e)))
        .collect::<Result<Vec<_>, _>>()?;

    // Validate the input array size
    if nums.len() != n {
        return Err(format!("Expected {} numbers, got {}", n, nums.len()).into());
    }

    // Compute the result using the count_valid_selections function
    let result = count_valid_selections(&nums);

    // Output the result
    println!("{}", result);

    Ok(())
}