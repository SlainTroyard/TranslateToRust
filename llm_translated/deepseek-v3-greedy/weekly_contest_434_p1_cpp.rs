use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Function to count the number of partitions
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            0
        } else {
            nums.len() as i32 - 1
        }
    }
}

fn main() {
    // Read the number of elements from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the array of integers from stdin
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        nums.push(num);
    }

    // Create an instance of Solution and compute the result
    let sol = Solution;
    let result = sol.count_partitions(nums);

    // Print the result to stdout
    println!("{}", result);
}