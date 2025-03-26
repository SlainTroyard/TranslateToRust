// Problem: Weekly Contest 434 Problem 1

use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // This function calculates the number of partitions based on the sum of the elements in the vector.
    fn count_partitions(nums: &Vec<i32>) -> i32 {
        let s: i32 = nums.iter().sum();
        if s % 2 != 0 {
            0
        } else {
            (nums.len() - 1) as i32
        }
    }
}

fn main() {
    // Read the number of elements from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    // Read the elements into a vector
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Create an instance of Solution and call the count_partitions method
    let sol = Solution;
    let result = sol.count_partitions(&nums);

    // Print the result to stdout
    println!("{}", result);
}