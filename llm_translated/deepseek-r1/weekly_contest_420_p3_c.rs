use std::io::{self, Read};
use std::i32;

fn min_operations(nums: &mut [i32]) -> i32 {
    let mut res = 0;

    if nums.len() == 1 {
        return res;
    }

    for i in (0..nums.len() - 1).rev() {
        if nums[i] > nums[i + 1] {
            let original = nums[i];
            let mut count = 1;

            loop {
                let current_num = nums[i];
                let max_factor = (current_num as f64).sqrt().floor() as i32 + 1;
                let mut max = i32::MIN;

                // Find the maximum proper divisor
                for j in 2..=max_factor {
                    if current_num % j == 0 {
                        max = max.max(j);
                        let other = current_num / j;
                        if current_num % other == 0 {
                            max = max.max(other);
                        }
                    }
                }

                // Check if no divisors found
                if max == i32::MIN {
                    return -1;
                }

                // Update count and check if product matches original
                count *= max;
                if count == original {
                    return -1;
                }

                // Apply division and increment operations
                nums[i] /= max;
                res += 1;

                // Check if current element is now valid
                if nums[i] <= nums[i + 1] {
                    break;
                }
            }
        }
    }

    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Parse input size and numbers
    let nums_size = tokens.next().and_then(|s| s.parse().ok()).expect("Invalid nums_size");
    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);

    for _ in 0..nums_size {
        let num = tokens.next().and_then(|s| s.parse().ok()).expect("Invalid number");
        nums.push(num);
    }

    // Compute and print result
    let result = min_operations(&mut nums);
    println!("{}", result);
}