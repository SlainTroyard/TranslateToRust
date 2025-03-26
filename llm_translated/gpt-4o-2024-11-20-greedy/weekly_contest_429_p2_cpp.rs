use std::collections::HashSet;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_distinct_elements(arr: &mut Vec<i32>, diff: i32) -> usize {
        let mut prev = i32::MIN;
        let mut c = HashSet::new();

        // Sort the array
        arr.sort();

        for &num in arr.iter() {
            // Calculate the maximum possible value for the current element
            let x = (prev + 1).max(num - diff);

            // Check if x is within the valid range
            if x <= num + diff {
                c.insert(x);
                prev = x;
            }
        }

        // Return the size of the set, which represents the count of distinct elements
        c.len()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array and the difference
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let n: usize = first_line_iter.next().unwrap().parse().unwrap();
    let diff: i32 = first_line_iter.next().unwrap().parse().unwrap();

    // Read the array elements
    let second_line = lines.next().unwrap().unwrap();
    let mut arr: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure the input array size matches the given size `n`
    assert_eq!(arr.len(), n);

    // Compute the result
    let result = Solution::max_distinct_elements(&mut arr, diff);

    // Output the result
    println!("{}", result);
}