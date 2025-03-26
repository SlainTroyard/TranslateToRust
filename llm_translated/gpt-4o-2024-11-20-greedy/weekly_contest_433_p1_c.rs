use std::io::{self, Write};
use std::cmp;

// Function to calculate the subarray sum
fn subarray_sum(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut ans = 0;
    let mut sums = vec![0; nums_size + 1]; // sums array with size nums_size + 1

    for i in 0..nums_size {
        sums[i + 1] = nums[i] + sums[i];
        ans += sums[i + 1] - sums[cmp::max(0, i as i32 - nums[i]) as usize];
    }

    ans
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Failed to parse n");

    let mut nums = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    nums.extend(
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Failed to parse number")),
    );

    // Ensure the input size matches the expected size
    assert_eq!(nums.len(), n, "Input size does not match n");

    // Calculate the result and print it
    let result = subarray_sum(&nums);
    println!("{}", result);
}