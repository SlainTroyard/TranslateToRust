use std::io::{self, BufRead};
use std::cmp::{max, min};

/// Minimum operations to make the array non-increasing
/// 
/// This function calculates the minimum number of operations needed to make
/// the array non-increasing, where each operation divides an element by one of its factors.
fn min_operations(nums: &mut [i32]) -> i32 {
    let mut res = 0;
    let nums_size = nums.len();

    // If there's only one element, no operations needed
    if nums_size == 1 {
        return res;
    }

    for i in (0..nums_size-1).rev() {
        if nums[i] > nums[i + 1] {
            let mut max_val: i32;
            let mut count = 1;
            let original = nums[i];
            
            loop {
                max_val = i32::MIN;
                let max_factor = (nums[i] as f64).sqrt() as i32 + 1;
                
                for j in 2..=max_factor {
                    if nums[i] % j == 0 {
                        max_val = max(max_val, j);
                        if nums[i] % (nums[i] / j) == 0 {
                            max_val = max(max_val, nums[i] / j);
                        }
                    }
                }
                
                if max_val == i32::MIN || (count *= max_val) == original {
                    return -1;
                } else {
                    nums[i] /= max_val;
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let nums_size: usize = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .trim()
        .parse()
        .expect("Failed to parse number");
    
    // Read the array elements
    let nums_input = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input");
    
    let mut nums: Vec<i32> = nums_input
        .split_whitespace()
        .take(nums_size)
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Calculate and print the result
    println!("{}", min_operations(&mut nums));
}