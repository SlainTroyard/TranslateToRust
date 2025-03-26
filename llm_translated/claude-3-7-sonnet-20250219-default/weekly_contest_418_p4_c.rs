use std::io::{self, BufRead};

fn gcd_values(nums: &[i32], queries: &[i64]) -> Vec<i32> {
    // Find the maximum value in nums
    let mx = *nums.iter().max().unwrap_or(&0);
    
    // Count occurrences of each number in nums
    let mut cnt_x = vec![0; mx as usize + 1];
    for &num in nums {
        cnt_x[num as usize] += 1;
    }
    
    // Calculate the number of pairs with gcd = i for each i from 1 to mx
    let mut cnt_gcd = vec![0i64; mx as usize + 1];
    for i in (1..=mx as usize).rev() {
        let mut c = 0;
        let mut j = i;
        while j <= mx as usize {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
            j += i;
        }
        cnt_gcd[i] += (c as i64 * (c - 1) as i64) / 2;
    }
    
    // Compute the prefix sum of cnt_gcd
    for i in 1..=mx as usize {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }
    
    // For each query, find the smallest value k such that cnt_gcd[k] > query
    let mut ans = vec![0; queries.len()];
    for (i, &query) in queries.iter().enumerate() {
        let mut left = 1;
        let mut right = mx as usize;
        while left < right {
            let mid = (left + right) / 2;
            if cnt_gcd[mid] <= query {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans[i] = left as i32;
    }
    
    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read numsSize
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read nums
    let nums: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read queriesSize
    let queries_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read queries
    let queries: Vec<i64> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate results
    let ans = gcd_values(&nums, &queries);
    
    // Print results
    for val in ans {
        print!("{} ", val);
    }
    println!();
    
    Ok(())
}