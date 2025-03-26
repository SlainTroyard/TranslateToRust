use std::io::{self, BufRead};

fn main() {
    // Read the number of elements in the array
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input");

    // Read the array elements
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    // Calculate and print the result
    let result = subarray_sum(&nums);
    println!("{}", result);
}

fn subarray_sum(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut s = vec![0; n + 1];

    // Compute the prefix sum array
    for i in 0..n {
        s[i + 1] = s[i] + nums[i];
    }

    let mut ans = 0;
    for i in 0..n {
        let start = if i as i32 - nums[i] >= 0 { i - nums[i] as usize } else { 0 };
        ans += s[i + 1] - s[start];
    }
    ans
}