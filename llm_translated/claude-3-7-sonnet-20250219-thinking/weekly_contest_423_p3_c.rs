// Problem: Weekly Contest 423 Problem 3
use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn sum_of_good_subsequences(nums: &mut [i32], nums_size: usize) -> i32 {
    let mut ans: i64 = 0;
    let mut cnt = vec![0i64; 100003];
    let mut sum = vec![0i64; 100003];
    
    for i in 0..nums_size {
        // Increment nums[i] by 1, similar to ++nums[i] in C
        nums[i] += 1;
        let x = nums[i] as usize;
        
        // Calculate c based on the counts of adjacent values
        let c = cnt[x - 1] + 1 + cnt[x + 1];
        cnt[x] = (cnt[x] + c) % MOD;
        
        // Calculate s based on sum of adjacent values and current count
        let s = c * (x as i64 - 1) + sum[x - 1] + sum[x + 1];
        sum[x] = (sum[x] + s) % MOD;
        
        // Add to the answer
        ans = (ans + s) % MOD;
    }
    
    ans as i32
}

fn main() -> io::Result<()> {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()
        .expect("Failed to parse the array size");
    
    // Allocate memory for the array, similar to malloc in C
    let mut nums = vec![0; n];
    
    // Read the elements of the array, one by one
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        nums[i] = input.trim().parse()
            .expect("Failed to parse array element");
    }
    
    // Call the function and output the result
    let result = sum_of_good_subsequences(&mut nums, n);
    println!("{}", result);
    
    Ok(())
}