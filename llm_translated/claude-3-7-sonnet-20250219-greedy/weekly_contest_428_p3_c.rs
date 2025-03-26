use std::io::{self, BufRead};
use std::cmp;

/// Calculates the number of beautiful splits in an array
fn beautiful_splits(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut res = 0;
    
    // KMP-like algorithm for pattern matching
    let mut cnt0 = vec![0; nums_size];
    let mut kmp_cnt = 0;
    
    // First pass: compute KMP table for the entire array
    cnt0[0] = 0;
    for i in 1..nums_size {
        while kmp_cnt > 0 && nums[i] != nums[kmp_cnt] {
            kmp_cnt = cnt0[kmp_cnt - 1];
        }
        if nums[i] == nums[kmp_cnt] {
            kmp_cnt += 1;
        }
        cnt0[i] = kmp_cnt;
        
        // Check for beautiful splits in the first part
        if i % 2 == 1 && ((i + 1) / 2) % (i + 1 - kmp_cnt) == 0 {
            res += (nums_size - i - 1) as i32;
        }
    }
    
    // Second pass: check for beautiful splits starting at each position
    for i in 1..nums_size {
        let mut cnt = vec![0; nums_size - i];
        let mut end = nums_size;
        kmp_cnt = 0;
        cnt[0] = 0;
        
        // Optimization to limit the search range
        if 2 * i < nums_size && (i % (2 * i - cnt0[2 * i - 1])) == 0 {
            end = cmp::min(end, 3 * i);
        }
        
        for j in i+1..end {
            while kmp_cnt > 0 && nums[j] != nums[i + kmp_cnt] {
                kmp_cnt = cnt[kmp_cnt - 1];
            }
            if nums[j] == nums[i + kmp_cnt] {
                kmp_cnt += 1;
            }
            cnt[j - i] = kmp_cnt;
            
            if (j - i + 1) % 2 == 0 && ((j - i + 1) / 2) % (j - i + 1 - kmp_cnt) == 0 {
                let len = j - i + 1;
                let div = i - 1 + len / 2;
                
                if len == i * 2 && (i % (div + 1 - cnt0[div])) == 0 {
                    break;
                }
                res += 1;
            }
        }
    }
    
    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the array elements
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print the result
    let result = beautiful_splits(&nums);
    println!("{}", result);
}