use std::io::{self, Read};

fn subarray_sum(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut sums = vec![0; nums_size + 1];
    
    // Compute prefix sums
    for i in 0..nums_size {
        sums[i + 1] = nums[i] + sums[i];
    }
    
    let mut ans = 0;
    // Calculate the sum of all valid subarrays
    for i in 0..nums_size {
        // Determine the start index using max(0, i - nums[i])
        let start = (i as i32 - nums[i]).max(0) as usize;
        ans += sums[i + 1] - sums[start];
    }
    
    ans
}

fn main() {
    // Read all input at once and split into whitespace-separated tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    
    // Parse the first token as the number of elements
    let n: usize = tokens.next()
        .expect("Missing number of elements")
        .parse()
        .expect("Invalid number format");
    
    // Parse the next n tokens as integers
    let nums: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().expect("Invalid integer format"))
        .collect();
    
    // Compute and print the result
    println!("{}", subarray_sum(&nums));
}