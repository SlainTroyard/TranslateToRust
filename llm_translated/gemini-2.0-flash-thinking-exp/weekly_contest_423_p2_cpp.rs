use std::cmp::{max, min};
use std::io;

fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut curr = 1;
    let mut ans = 0;

    if nums.is_empty() {
        return 0;
    }

    // Traverse through the nums array
    for i in 1..nums.len() {
        if nums[i - 1] < nums[i] {
            curr += 1; // Increase the length of the current increasing subarray
        } else {
            prev = curr; // Update the previous subarray length
            curr = 1;    // Reset current subarray length
        }
        // Update the answer by considering both the current and previous subarray lengths
        ans = max(ans, max(curr / 2, min(prev, curr)));
    }
    max(ans, max(curr / 2, min(prev, curr))) // Handle the last subarray
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Invalid input for n");

    let mut nums: Vec<i32> = Vec::new();
    for _ in 0..n {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read line");
        let num: i32 = num_str.trim().parse().expect("Invalid input for number");
        nums.push(num);
    }

    let result = max_increasing_subarrays(nums);
    println!("{}", result);
}