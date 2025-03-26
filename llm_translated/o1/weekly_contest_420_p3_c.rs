// LeetCode Weekly Contest 420 Problem 3 in Rust
// ---------------------------------------------
// This program reads an integer N (numsSize) from stdin,
// then reads N integers (nums), and prints the result of
// the min_operations(...) function to stdout.
// The I/O format matches the C version exactly.

use std::io::{self, BufRead};

fn min_operations(nums: &mut [i32]) -> i32 {
    // If there's only one element, no operations are needed
    if nums.len() == 1 {
        return 0;
    }

    let mut res = 0;

    // Iterate backwards from numsSize - 2 down to 0
    for i in (0..nums.len() - 1).rev() {
        if nums[i] > nums[i + 1] {
            let original = nums[i];
            let mut count = 1;

            // Repeatedly find the largest divisor until nums[i] <= nums[i+1] or fail
            loop {
                let mut max_divisor = i32::MIN;

                // Calculate an upper bound for checking divisors
                let max_factor = (nums[i] as f64).sqrt() as i32 + 1;

                // Find the largest divisor of nums[i]
                for j in 2..=max_factor {
                    if nums[i] % j == 0 {
                        if j > max_divisor {
                            max_divisor = j;
                        }
                        let other_div = nums[i] / j;
                        if nums[i] % other_div == 0 && other_div > max_divisor {
                            max_divisor = other_div;
                        }
                    }
                }

                // If no divisor was found or multiplying leads to the original value, fail early
                if max_divisor == i32::MIN {
                    return -1;
                }
                count *= max_divisor;
                if count == original {
                    return -1;
                }

                // Divide nums[i] by the largest divisor found
                nums[i] /= max_divisor;

                // If we've made nums[i] <= nums[i+1], increment operation count and break
                if nums[i] <= nums[i + 1] {
                    res += 1;
                    break;
                }
            }
        }
    }

    res
}

fn main() {
    // Read the number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read numsSize");
    let nums_size: usize = input.trim().parse().expect("Failed to parse numsSize");

    // Read the array of integers
    let mut nums = Vec::with_capacity(nums_size);
    let stdin = io::stdin();
    let mut read_count = 0;
    for line in stdin.lock().lines() {
        let line_str = line.expect("Failed to read line");
        for num_str in line_str.split_whitespace() {
            if read_count >= nums_size {
                break;
            }
            let val: i32 = num_str.parse().expect("Failed to parse integer");
            nums.push(val);
            read_count += 1;
        }
        if read_count >= nums_size {
            break;
        }
    }

    // Calculate and print the result
    let result = min_operations(&mut nums);
    println!("{}", result);
}