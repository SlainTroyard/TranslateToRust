use std::io::{self, Write};
use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn max_good_number(nums: &mut Vec<i32>) -> i32 {
        // Sort nums based on custom comparator
        nums.sort_unstable_by(|&a, &b| {
            let len_a = (a as f32).log2().floor() as i32 + 1;
            let len_b = (b as f32).log2().floor() as i32 + 1;

            let combined_ab = (a << len_b) | b;
            let combined_ba = (b << len_a) | a;

            // We want descending order
            combined_ba.cmp(&combined_ab)
        });

        // Calculate the "max good number" by combining all numbers in sorted order
        let mut ans = 0;
        for &x in nums {
            let len_x = (x as f32).log2().floor() as i32 + 1;
            ans = (ans << len_x) | x;
        }
        ans
    }
}

fn main() {
    // Read input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Parse the size of the array
    let num_size: usize = input.trim().parse().unwrap();

    // Read the next line for the numbers
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    assert_eq!(nums.len(), num_size); // Ensure input length matches the declared size

    // Solve the problem
    let result = Solution::max_good_number(&mut nums);

    // Print the result
    println!("{}", result);
}