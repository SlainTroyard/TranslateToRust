use std::io::{self, Read};

fn minimum_sum_subarray(nums: &[i32], l: usize, r: usize) -> i32 {
    let mut min_sum: Option<i32> = None;

    for window_size in l..=r {
        let mut sum = 0;
        for (i, &num) in nums.iter().enumerate() {
            sum += num;
            if i >= window_size {
                sum -= nums[i - window_size];
            }

            // Ensure the sum is considered only for valid subarray sizes
            if i >= window_size - 1 && sum > 0 {
                if min_sum.is_none() || min_sum.unwrap() > sum {
                    min_sum = Some(sum);
                }
            }
        }
    }

    // Return -1 if no valid subarray was found, otherwise unwrap the minimum sum
    min_sum.unwrap_or(-1)
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Parse the input
    let mut lines = input.split_whitespace();
    let nums_size: usize = lines.next().unwrap().parse().unwrap();

    // Parse the array elements
    let nums: Vec<i32> = lines
        .by_ref()
        .take(nums_size)
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    // Parse the l and r values
    let l: usize = lines.next().unwrap().parse().unwrap();
    let r: usize = lines.next().unwrap().parse().unwrap();

    // Call the function
    let result = minimum_sum_subarray(&nums, l, r);

    // Output the result
    println!("{}", result);
}