use std::io::{self, BufRead};

fn minimum_sum_subarray(nums: &[i32], l: i32, r: i32) -> i32 {
    let nums_size = nums.len();
    let mut min_sum: Option<i32> = None;
    
    for length in l..=r {
        let mut sum = 0;
        for i in 0..nums_size {
            sum += nums[i];
            if i as i32 >= length {
                sum -= nums[i - length as usize];
            }
            if sum > 0 && i as i32 >= length - 1 {
                if let Some(current_min) = min_sum {
                    if current_min > sum {
                        min_sum = Some(sum);
                    }
                } else {
                    min_sum = Some(sum);
                }
            }
        }
    }
    
    min_sum.unwrap_or(-1)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input the size of the array
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Input the array elements
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Input the range [l, r]
    let lr: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let l = lr[0];
    let r = lr[1];
    
    // Call the function
    let result = minimum_sum_subarray(&nums, l, r);
    
    // Output the result
    println!("{}", result);
}