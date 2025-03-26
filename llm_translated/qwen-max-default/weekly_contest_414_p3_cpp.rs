// Problem: Weekly Contest 414 Problem 3

use std::io;

fn main() {
    // Read the size of the input vector
    let mut nums_size = String::new();
    io::stdin().read_line(&mut nums_size).expect("Failed to read line");
    let nums_size: usize = nums_size.trim().parse().expect("Please type a number!");

    // Read the elements of the vector
    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed to read line");
        nums.push(num.trim().parse().expect("Please type a number!"));
    }

    // Create an instance of the Solution struct and call the find_maximum_score method
    let sol = Solution;
    let result = sol.find_maximum_score(&nums);

    // Print the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    /// Finds the maximum score based on the given algorithm.
    ///
    /// # Arguments
    ///
    /// * `nums` - A reference to a vector of integers.
    ///
    /// # Returns
    ///
    /// * The maximum score as a 64-bit integer.
    pub fn find_maximum_score(nums: &Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut mx: i32 = 0;
        for &num in nums.iter().take(nums.len() - 1) {
            mx = mx.max(num);
            ans += mx as i64;
        }
        ans
    }
}