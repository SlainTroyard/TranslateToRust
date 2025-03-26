use std::io;
use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn max_good_number(nums: &mut Vec<i32>) -> i32 {
        // Sort the numbers with a comparator similar to the CPP version
        nums.sort_unstable_by(|&a, &b| {
            let len_a = (32 - a.leading_zeros()) as usize; // Equivalent to `__lg(a) + 1`
            let len_b = (32 - b.leading_zeros()) as usize; // Equivalent to `__lg(b) + 1`
            
            let a_with_b = (a as i64) << len_b | b as i64; // Shift and append b after a
            let b_with_a = (b as i64) << len_a | a as i64; // Shift and append a after b

            // Sort in descending order, so we reverse the comparison
            b_with_a.cmp(&a_with_b)
        });

        let mut ans = 0;
        for &x in nums.iter() {
            let len_x = (32 - x.leading_zeros()) as i32; // Equivalent to `__lg(x) + 1`
            ans = (ans << len_x) | x; // Shift ans and append x
        }
        ans
    }
}

fn main() {
    // Reading input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse input for the number of integers (unused in Rust as Vec is dynamic)
    let num_size: usize = input.trim().parse().expect("Failed to parse numSize");
    input.clear();

    // Read the list of integers
    io::stdin().read_line(&mut input).expect("Failed to read nums");
    let mut nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse an integer"))
        .collect();

    // Ensure the input size matches what was declared (optional, for safety)
    assert_eq!(nums.len(), num_size, "Input size mismatch");

    // Process the input using the solution
    let result = Solution::max_good_number(&mut nums);

    // Output the result
    println!("{}", result);
}