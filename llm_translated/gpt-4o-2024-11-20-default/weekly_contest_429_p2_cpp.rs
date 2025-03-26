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
            // Compute the value `x` as max(prev + 1, arr[i] - diff)
            let x = (prev + 1).max(num - diff);

            // Ensure `x` falls within the acceptable range [arr[i] - diff, arr[i] + diff]
            if x <= num + diff {
                c.insert(x);
                prev = x;
            }
        }

        // The size of the set `c` is the result, as it contains all distinct `x` values
        c.len()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array and the difference
    let first_line = lines.next().unwrap().unwrap();
    let mut input_iter = first_line.split_whitespace();
    let n: usize = input_iter.next().unwrap().parse().unwrap();
    let diff: i32 = input_iter.next().unwrap().parse().unwrap();

    // Read the array elements
    let second_line = lines.next().unwrap().unwrap();
    let mut arr: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure the array has the expected number of elements
    assert_eq!(arr.len(), n);

    // Compute the result
    let result = Solution::max_distinct_elements(&mut arr, diff);

    // Output the result
    println!("{}", result);
}