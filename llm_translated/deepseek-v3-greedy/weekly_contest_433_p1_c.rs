use std::io::{self, Write};

// Helper function to calculate the maximum of two values
fn max(base: i32, n: i32) -> i32 {
    if n > base { n } else { base }
}

// Function to calculate the sum of all subarrays
fn subarray_sum(nums: &[i32]) -> i32 {
    let mut ans = 0;
    let mut sums = vec![0; nums.len() + 1];
    
    for i in 0..nums.len() {
        sums[i + 1] = nums[i] + sums[i];
        ans += sums[i + 1] - sums[max(0, i as i32 - nums[i]) as usize];
    }
    
    ans
}

fn main() {
    // Read the number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid number");
    
    // Read the array elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please enter a valid number");
        nums.push(num);
    }
    
    // Calculate and print the result
    let result = subarray_sum(&nums);
    println!("{}", result);
}