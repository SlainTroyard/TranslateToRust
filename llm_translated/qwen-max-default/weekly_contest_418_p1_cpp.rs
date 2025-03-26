// Problem: Weekly Contest 418 Problem 1

use std::io;

fn main() {
    // Read the number of elements
    let mut num_size = String::new();
    io::stdin().read_line(&mut num_size).expect("Failed to read line");
    let num_size: usize = num_size.trim().parse().expect("Please type a number!");

    // Read the elements into a vector
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Create an instance of Solution and call max_good_number
    let solution = Solution;
    let result = solution.max_good_number(nums);

    // Print the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    /// Sorts the numbers and constructs the maximum good number.
    pub fn max_good_number(mut nums: Vec<i32>) -> i32 {
        // Custom sort function
        nums.sort_by(|&a, &b| {
            let len_a = (a as f64).log2().floor() as u32 + 1;
            let len_b = (b as f64).log2().floor() as u32 + 1;
            ((a << len_b) | b).cmp(&((b << len_a) | a))
        });

        // Construct the maximum good number
        let mut ans = 0;
        for x in nums {
            let len_x = (x as f64).log2().floor() as u32 + 1;
            ans = (ans << len_x) | x;
        }
        ans
    }
}