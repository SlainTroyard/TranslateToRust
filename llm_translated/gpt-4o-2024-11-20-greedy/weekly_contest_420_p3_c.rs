use std::io::{self, BufRead};
use std::cmp;
use std::usize;

// Function to calculate the minimum operations
fn min_operations(nums: &mut Vec<i32>) -> i32 {
    let mut res = 0;

    if nums.len() == 1 {
        return res;
    }

    for i in (0..nums.len() - 1).rev() {
        if nums[i] > nums[i + 1] {
            let mut original = nums[i];
            let mut count = 1;

            loop {
                let mut max = i32::MIN;
                let max_factor = (nums[i] as f64).sqrt() as i32 + 1;

                for j in 2..=max_factor {
                    if nums[i] % j == 0 {
                        max = cmp::max(max, j);
                        if nums[i] % (nums[i] / j) == 0 {
                            max = cmp::max(max, nums[i] / j);
                        }
                    }
                }

                if max == i32::MIN || (count *= max) == original {
                    return -1;
                } else {
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
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line contains the size of the array
    let nums_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse numsSize");

    // Second line contains the array elements
    let mut nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse number"))
        .collect();

    // Ensure the input size matches the declared size
    assert_eq!(nums.len(), nums_size, "Input size does not match numsSize");

    // Calculate the result and print it
    let result = min_operations(&mut nums);
    println!("{}", result);
}