// Problem: Weekly Contest 433 Problem 1

use std::io;

fn subarray_sum(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    let mut sums = vec![0; n + 1];

    for i in 0..n {
        sums[i + 1] = nums[i] + sums[i];
        let start = if i >= nums[i] as usize { i - nums[i] as usize } else { 0 };
        ans += sums[i + 1] - sums[start];
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    // Read the array of integers
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let num: i32 = input.trim().parse()?;
        nums.push(num);
    }

    // Calculate and print the result
    let result = subarray_sum(&nums);
    println!("{}", result);

    Ok(())
}