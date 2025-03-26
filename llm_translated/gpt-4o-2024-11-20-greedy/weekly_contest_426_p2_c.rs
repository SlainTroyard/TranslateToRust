use std::io::{self, BufRead};
use std::cmp;
use std::collections::HashMap;

fn get_largest_outlier(nums: &[i32]) -> i32 {
    let total_sum: i32 = nums.iter().sum();

    // Create a frequency map for the numbers
    let mut set = HashMap::new();
    for &num in nums {
        *set.entry(num).or_insert(0) += 1;
    }

    let mut ans = -1001; // Initialize answer to the smallest possible value
    for &num in nums {
        let curr_sum = total_sum - num;
        if curr_sum % 2 == 0 { // Check if curr_sum is even
            let half = curr_sum / 2;
            let threshold = if half == num { 1 } else { 0 };

            if (-1000..=1000).contains(&half) {
                if set.get(&half).unwrap_or(&0) > &threshold {
                    ans = cmp::max(ans, num);
                }
            }
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let nums_size: usize = lines.next()
        .expect("Expected input for numsSize")
        .expect("Failed to read numsSize")
        .trim()
        .parse()
        .expect("Failed to parse numsSize");

    // Read the elements of the array
    let nums: Vec<i32> = lines.next()
        .expect("Expected input for nums")
        .expect("Failed to read nums")
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    assert_eq!(nums.len(), nums_size, "Input size does not match numsSize");

    // Calculate the largest outlier
    let result = get_largest_outlier(&nums);

    // Output the result
    println!("{}", result);
}