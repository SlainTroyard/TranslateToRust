use std::io::{self, BufRead};

fn subarray_sum(nums: &[i32]) -> i32 {
    let mut ans = 0;
    let nums_size = nums.len();
    let mut sums = vec![0; nums_size + 1]; // Equivalent to C's `int sums[numsSize+1]`

    for i in 0..nums_size {
        sums[i + 1] = nums[i] + sums[i]; // C's `sums[i+1] = nums[i] + sums[i]`
        
        // max(0, i - nums[i]), equivalent to C's `sums[i+1] - sums[max(0, i-nums[i])]`
        let max_base = if i as i32 - nums[i] > 0 { i as i32 - nums[i] } else { 0 };
        ans += sums[i + 1] - sums[max_base as usize];
    }

    ans
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n (number of elements in the array)
    let n: usize = lines
        .next()
        .expect("No input provided")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Invalid number format");
    
    // Read the array of numbers
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected a second line of input")
        .expect("Failed to read line")
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number format"))
        .collect();

    // Guard against mismatched input size
    assert_eq!(nums.len(), n, "Input size mismatch");

    // Call the translated function and compute the result
    let result = subarray_sum(&nums);

    // Output the result
    println!("{}", result);
}