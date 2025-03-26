use std::io::{self, BufRead};
use std::cmp::Ordering;

fn max_distinct_elements(mut arr: Vec<i32>, diff: i32) -> i32 {
    let mut prev = i32::MIN; // Equivalent to INT_MIN
    let mut distinct_count = 0;

    // Sort the array in ascending order
    arr.sort_unstable();

    for &num in &arr {
        // Calculate the smallest possible value for `x` within the allowed range
        let x = if prev + 1 > num - diff { prev + 1 } else { num - diff };

        // If `x` is still within the bounds of `num +/- diff`, update `prev` and count it
        if x <= num + diff {
            distinct_count += 1;
            prev = x;
        }
    }

    distinct_count
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // Read the size of the array `n` and the difference `diff`
    let n_diff_line = iterator.next().unwrap().unwrap();
    let parts: Vec<i32> = n_diff_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = parts[0];
    let diff = parts[1];

    // Read the array elements
    let arr_line = iterator.next().unwrap().unwrap();
    let arr: Vec<i32> = arr_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Compute the result
    let result = max_distinct_elements(arr, diff);

    // Output the result
    println!("{}", result);
}