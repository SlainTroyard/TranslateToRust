use std::io::{self, Read};

fn subarray_sum(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    // Compute prefix sums, starting with 0 followed by cumulative sums
    let mut s = vec![0];
    let mut sum = 0;
    for &num in &nums {
        sum += num;
        s.push(sum);
    }
    
    let mut ans = 0;
    for i in 0..n {
        // Calculate the starting index using the same logic as the original C++ code
        let start = (i as i32 - nums[i]).max(0) as usize;
        ans += s[i + 1] - s[start];
    }
    ans
}

fn main() {
    // Read all input at once and split into whitespace-separated tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    
    // Parse n (first token) and then exactly n integers
    let n: usize = tokens
        .next()
        .and_then(|s| s.parse().ok())
        .expect("Invalid input for n");
    
    let nums: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().expect("Invalid integer in input"))
        .collect();
    
    // Ensure we have exactly n numbers as per original logic
    assert_eq!(nums.len(), n, "Input does not contain exactly {} numbers", n);
    
    println!("{}", subarray_sum(nums));
}