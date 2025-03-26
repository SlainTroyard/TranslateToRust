use std::io::{self, Write}; // For input/output handling
use std::cmp;

// Function to calculate minimum operations
fn min_operations(nums: &mut Vec<i32>) -> i32 {
    let mut res = 0;

    // Handle case when numsSize is 1
    if nums.len() == 1 {
        return res;
    }

    // Iterate over the array from the second last element to the beginning
    for i in (0..nums.len() - 1).rev() {
        if nums[i] > nums[i + 1] {
            let mut original = nums[i];
            let mut count = 1;

            // Enter loop to find the smallest divisor greater than 1 until the condition is satisfied
            loop {
                let mut max = i32::MIN;

                // Find divisors of nums[i]
                let max_factor = (nums[i] as f64).sqrt().floor() as i32 + 1;
                for j in 2..=max_factor {
                    if nums[i] % j == 0 {
                        max = cmp::max(max, j); // Maximum divisor

                        if nums[i] % (nums[i] / j) == 0 {
                            max = cmp::max(max, nums[i] / j);
                        }
                    }
                }

                // If no divisor is found or we come back to the original number, return -1
                if max == i32::MIN || (count * max == original) {
                    return -1;
                } else {
                    // Otherwise, divide the number and increment res if condition satisfied
                    nums[i] /= max;
                    if nums[i] <= nums[i + 1] {
                        res += 1;
                        break;
                    }
                }
            }
        }
    }

    res
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums_size: usize = input.trim().parse().unwrap();

    // Read nums array
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure the number of elements matches nums_size
    assert_eq!(nums.len(), nums_size, "Input size does not match numsSize.");

    // Call the function and print the result
    let result = min_operations(&mut nums);
    println!("{}", result);
}