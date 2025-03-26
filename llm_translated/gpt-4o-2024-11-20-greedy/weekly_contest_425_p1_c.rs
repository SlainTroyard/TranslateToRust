use std::io::{self, Write};

fn minimum_sum_subarray(nums: &[i32], l: usize, r: usize) -> i32 {
    let mut min_sum: Option<i32> = None;

    // Iterate over the range [l, r]
    for window_size in l..=r {
        let mut sum = 0;

        // Sliding window approach
        for (i, &num) in nums.iter().enumerate() {
            sum += num;

            // Remove the element that is sliding out of the window
            if i >= window_size {
                sum -= nums[i - window_size];
            }

            // Check if the current window is valid and update min_sum
            if i >= window_size - 1 && sum > 0 {
                if min_sum.is_none() || min_sum.unwrap() > sum {
                    min_sum = Some(sum);
                }
            }
        }
    }

    // Return the result, -1 if no valid subarray was found
    min_sum.unwrap_or(-1)
}

fn main() {
    let mut input = String::new();

    // Read the size of the array
    io::stdin().read_line(&mut input).unwrap();
    let nums_size: usize = input.trim().parse().unwrap();

    // Read the array elements
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read the range [l, r]
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let range: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (l, r) = (range[0], range[1]);

    // Call the function
    let result = minimum_sum_subarray(&nums, l, r);

    // Output the result
    println!("{}", result);
}