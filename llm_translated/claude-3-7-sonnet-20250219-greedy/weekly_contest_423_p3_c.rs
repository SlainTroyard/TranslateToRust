use std::io::{self, BufRead};

fn sum_of_good_subsequences(nums: &mut [i32]) -> i32 {
    let mut cnt = vec![0i64; 100003];
    let mut sum = vec![0i64; 100003];
    let mut ans = 0i64;
    let modulo: i64 = 1_000_000_007;

    for i in 0..nums.len() {
        // Increment nums[i] by 1 and store in x
        let x = {
            nums[i] += 1;
            nums[i] as usize
        };
        
        // Calculate new count for x
        let c = (cnt[x - 1] + 1 + cnt[x + 1]) % modulo;
        cnt[x] = (cnt[x] + c) % modulo;
        
        // Calculate new sum for x
        let s = (c * (x as i64 - 1) + sum[x - 1] + sum[x + 1]) % modulo;
        sum[x] = (sum[x] + s) % modulo;
        
        // Update answer
        ans = (ans + s) % modulo;
    }
    
    ans as i32
}

fn main() {
    // Read the size of the array
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let n: usize = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .trim()
        .parse()
        .expect("Failed to parse n");
    
    // Read the elements of the array
    let nums_line = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input");
    
    let mut nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Call the function and output the result
    let result = sum_of_good_subsequences(&mut nums);
    println!("{}", result);
}