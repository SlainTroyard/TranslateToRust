use std::io::{self, BufRead};
use std::cmp;

// Function to calculate the largest outlier from the given array
fn get_largest_outlier(nums: Vec<i32>) -> i32 {
    let nums_size = nums.len() as i32;
    let total_sum: i32 = nums.iter().sum();

    // Array to count occurrences of numbers in the range [-1000, 1000]
    let mut set = vec![0; 2001]; // Range from -1000 to 1000 (shifted by +1000)
    for &num in &nums {
        set[(num + 1000) as usize] += 1;
    }

    let mut ans = -1001; // Initialize result with smallest number outside of valid range

    for &num in &nums {
        let curr_sum = total_sum - num;
        if curr_sum % 2 == 0 { // curr_sum is even
            let half = curr_sum / 2;
            let threshold = if half == num { 1 } else { 0 };

            // Check if `half` is within the range [-1000, 1000]
            if half >= -1000 && half <= 1000 {
                // If we have more than `threshold` occurrences of `half`, update ans
                if set[(half + 1000) as usize] > threshold {
                    ans = cmp::max(ans, num);
                }
            }
        }
    }

    ans
}

fn main() {
    // Read input from stdin and process it
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the size of the array
    let nums_size = lines.next()
        .and_then(|line| line.ok())
        .and_then(|line| line.parse::<usize>().ok())
        .expect("Invalid input for numsSize");

    // Parse the array elements
    let nums: Vec<i32> = lines.next()
        .and_then(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        })
        .expect("Invalid input for nums array");

    assert_eq!(nums.len(), nums_size, "nums array size does not match numsSize");

    // Calculate the largest outlier
    let result = get_largest_outlier(nums);

    // Output the result
    println!("{}", result);
}