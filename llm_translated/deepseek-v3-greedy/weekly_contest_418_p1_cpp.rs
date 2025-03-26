use std::cmp::Ordering;
use std::io::{self, BufRead};

fn main() {
    // Read the size of the vector from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let num_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the vector of integers from stdin
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let num: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        nums.push(num);
    }

    // Create an instance of the Solution and compute the result
    let solution = Solution;
    let result = solution.max_good_number(nums);

    // Print the result to stdout
    println!("{}", result);
}

struct Solution;

impl Solution {
    fn max_good_number(&self, mut nums: Vec<i32>) -> i32 {
        // Sort the numbers based on the custom comparator
        nums.sort_by(|a, b| {
            let len_a = (a.leading_zeros() as i32 - 31).max(0) + 1;
            let len_b = (b.leading_zeros() as i32 - 31).max(0) + 1;
            let ab = (a << len_b) | b;
            let ba = (b << len_a) | a;
            ba.cmp(&ab)
        });

        // Combine the numbers to form the maximum good number
        let mut ans = 0;
        for x in nums {
            let len_x = (x.leading_zeros() as i32 - 31).max(0) + 1;
            ans = (ans << len_x) | x;
        }
        ans
    }
}