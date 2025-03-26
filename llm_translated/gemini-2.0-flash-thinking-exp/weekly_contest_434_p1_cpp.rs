use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn count_partitions(&self, nums: &Vec<i32>) -> i32 {
        // Calculate the sum of the numbers in the vector
        let s: i32 = nums.iter().sum();
        // Return 0 if the sum is odd, otherwise return the size of the vector minus 1
        if s % 2 != 0 {
            0
        } else {
            (nums.len() as i32) - 1
        }
    }
}

fn main() {
    // Get standard input
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // Read the first line to get the value of n
    let n_str = iterator.next().unwrap().unwrap();
    let n: i32 = n_str.trim().parse().unwrap();

    // Create a vector to store the numbers
    let mut nums: Vec<i32> = Vec::new();

    // Read n lines to get the numbers and store them in the vector
    for _i in 0..n {
        let num_str = iterator.next().unwrap().unwrap();
        let num: i32 = num_str.trim().parse().unwrap();
        nums.push(num);
    }

    // Create an instance of the Solution struct
    let sol = Solution {};
    // Call the count_partitions function and print the result to standard output
    println!("{}", sol.count_partitions(&nums));
}