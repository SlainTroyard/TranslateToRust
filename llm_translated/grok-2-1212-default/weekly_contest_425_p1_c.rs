use std::io::{self, BufRead};

fn minimum_sum_subarray(nums: &[i32], nums_size: usize, l: usize, r: usize) -> i32 {
    let mut min_sum = None;
    for window_start in l..=r {
        let mut sum = 0;
        for i in 0..nums_size {
            sum += nums[i];
            if i >= window_start {
                sum -= nums[i - window_start];
            }
            if sum > 0 && i >= window_start - 1 {
                if min_sum.is_none() || min_sum.unwrap() > sum {
                    min_sum = Some(sum);
                }
            }
        }
    }
    min_sum.unwrap_or(-1)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the size of the array
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Input the array elements
    let nums: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Input the range [l, r]
    let lr: Vec<usize> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let l = lr[0];
    let r = lr[1];

    // Call the function
    let result = minimum_sum_subarray(&nums, nums_size, l, r);

    // Output the result
    println!("{}", result);

    Ok(())
}