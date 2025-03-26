use std::io::{self, BufRead};

fn sum_of_good_subsequences(nums: &mut [i32]) -> i32 {
    let mut ans: i64 = 0;
    let mut cnt = vec![0i64; 100003];
    let mut sum = vec![0i64; 100003];
    
    let modulo: i64 = 1_000_000_007;
    
    for i in 0..nums.len() {
        nums[i] += 1; // increment the number as in the original code
        let x = nums[i] as usize;
        
        // Calculate c: cnt[x-1] + 1 + cnt[x+1]
        let c = (cnt[x - 1] + 1 + cnt[x + 1]) % modulo;
        
        // Update cnt[x]
        cnt[x] = (cnt[x] + c) % modulo;
        
        // Calculate s: c * (x-1) + sum[x-1] + sum[x+1]
        let s = (c * ((x - 1) as i64) + sum[x - 1] + sum[x + 1]) % modulo;
        
        // Update sum[x]
        sum[x] = (sum[x] + s) % modulo;
        
        // Update answer
        ans = (ans + s) % modulo;
    }
    
    ans as i32
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let n: usize = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse input size");
    
    // Read the elements of the array
    let mut nums: Vec<i32> = lines.next().unwrap()?.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Call the function and output the result
    let result = sum_of_good_subsequences(&mut nums);
    println!("{}", result);
    
    Ok(())
}