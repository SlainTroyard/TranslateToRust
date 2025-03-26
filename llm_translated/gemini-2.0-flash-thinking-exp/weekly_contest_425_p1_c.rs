use std::io;
use std::io::BufRead;

fn minimum_sum_subarray(nums: &[i32], l: i32, r: i32) -> i32 {
    let mut min_sum: Option<i32> = None;
    for current_l in l..=r {
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            if i as i32 >= current_l {
                sum -= nums[i - current_l as usize];
            }
            if sum > 0 && i as i32 >= current_l - 1 {
                if min_sum.is_none() || min_sum.unwrap() > sum {
                    min_sum = Some(sum);
                }
            }
        }
    }
    match min_sum {
        Some(val) => val,
        None => -1,
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the size of the array
    let nums_size: usize = lines.next().unwrap().expect("Failed to read line").trim().parse().expect("Failed to parse numsSize");

    // Allocate memory for the array (Vec in Rust) and input the array elements
    let nums_line = lines.next().unwrap().expect("Failed to read line");
    let nums: Vec<i32> = nums_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse num"))
        .collect();
    assert_eq!(nums.len(), nums_size, "Number of input numbers does not match numsSize");


    // Input the range [l, r]
    let lr_line = lines.next().unwrap().expect("Failed to read line");
    let lr_vals: Vec<i32> = lr_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse l or r"))
        .collect();
    let l = lr_vals[0];
    let r = lr_vals[1];

    // Call the function
    let result = minimum_sum_subarray(&nums, l, r);

    // Output the result
    println!("{}", result);
}