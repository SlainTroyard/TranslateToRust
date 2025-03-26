// Problem: Weekly Contest 414 Problem 3

use std::io;

struct Solution;

impl Solution {
    // Find the maximum score based on the given algorithm
    fn find_maximum_score(nums: &Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut mx: i32 = 0;
        for &num in nums.iter().take(nums.len() - 1) {
            mx = mx.max(num);
            ans += mx as i64;
        }
        ans
    }
}

fn main() {
    // Read the size of the nums vector from stdin
    let mut nums_size = String::new();
    io::stdin().read_line(&mut nums_size).expect("Failed to read line");
    let nums_size: usize = nums_size.trim().parse().expect("Please type a number!");

    // Read the elements of the nums vector from stdin
    let mut nums: Vec<i32> = vec![0; nums_size];
    for i in 0..nums_size {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read line");
        nums[i] = num_str.trim().parse().expect("Please type a number!");
    }

    // Create an instance of Solution and call the find_maximum_score method
    let sol = Solution;
    println!("{}", sol.find_maximum_score(&nums));
}