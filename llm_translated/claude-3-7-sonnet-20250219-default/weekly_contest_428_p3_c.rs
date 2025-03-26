use std::io::{self, BufRead};
use std::cmp;

/// Calculates the number of "beautiful splits" in an array
///
/// This is a direct translation of the C implementation for LeetCode Weekly Contest 428 Problem 3
fn beautiful_splits(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut res = 0;
    let mut cnt0 = vec![0; n];
    let mut kmpcnt = 0;
    
    // First pass: compute KMP-like array
    cnt0[0] = 0;
    for i in 1..n {
        while kmpcnt > 0 && nums[i] != nums[kmpcnt] {
            kmpcnt = cnt0[kmpcnt - 1];
        }
        if nums[i] == nums[kmpcnt] {
            kmpcnt += 1;
        }
        cnt0[i] = kmpcnt;
        if i % 2 == 1 && ((i + 1) / 2) % (i + 1 - kmpcnt) == 0 {
            res += (n - i - 1) as i32;
        }
    }
    
    // Second pass: check for beautiful splits
    for i in 1..n {
        let mut cnt = vec![0; n - i];
        let mut end = n;
        kmpcnt = 0;
        cnt[0] = 0;
        
        if 2 * i < n && (i % (2 * i - cnt0[2 * i - 1])) == 0 {
            end = cmp::min(end, 3 * i);
        }
        
        for j in (i + 1)..end {
            while kmpcnt > 0 && nums[j] != nums[i + kmpcnt] {
                kmpcnt = cnt[kmpcnt - 1];
            }
            if nums[j] == nums[i + kmpcnt] {
                kmpcnt += 1;
            }
            cnt[j - i] = kmpcnt;
            
            if (j - i + 1) % 2 == 0 && ((j - i + 1) / 2) % (j - i + 1 - kmpcnt) == 0 {
                let len = j - i + 1;
                let div = i - 1 + len / 2;
                
                if len == i * 2 && (i % (div + 1 - cnt0[div])) == 0 {
                    break;
                }
                
                res += 1;
            }
        }
    }
    
    return res;
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let n: usize = lines.next().unwrap()?.trim().parse().expect("Invalid input for array size");
    
    // Read the array elements
    let nums_line = lines.next().unwrap()?;
    let nums: Vec<i32> = nums_line.trim()
                                  .split_whitespace()
                                  .map(|s| s.parse().expect("Invalid input for array element"))
                                  .collect();
    
    // Calculate and print the result
    let result = beautiful_splits(&nums);
    println!("{}", result);
    
    Ok(())
}