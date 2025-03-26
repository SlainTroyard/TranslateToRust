use std::io::{self, BufRead};
use std::cmp::Ordering;

fn main() {
    // Read the number of elements from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let num_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the list of numbers
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let num: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        nums.push(num);
    }

    // Create an instance of the solution and compute the result
    let result = max_good_number(nums);

    // Print the result
    println!("{}", result);
}

fn max_good_number(mut nums: Vec<i32>) -> i32 {
    // Sort the numbers based on the custom comparator
    nums.sort_by(|a, b| {
        let len_a = a.leading_zeros().count_ones() as i32;
        let len_b = b.leading_zeros().count_ones() as i32;
        let a_combined = (a << len_b) | b;
        let b_combined = (b << len_a) | a;
        b_combined.cmp(&a_combined)
    });

    // Combine the sorted numbers into the final result
    let mut ans = 0;
    for x in nums {
        let len_x = x.leading_zeros().count_ones() as i32;
        ans = (ans << len_x) | x;
    }
    ans
}