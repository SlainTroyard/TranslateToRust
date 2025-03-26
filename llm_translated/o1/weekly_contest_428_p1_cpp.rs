// Problem: Weekly Contest 428 Problem 1
//
// This Rust program reads an integer n from stdin, then reads n lines of events
// where each line contains two integers: [index, time]. We then compute the button
// index with the longest time difference according to the given logic and print it.

use std::error::Error;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Translated logic from C++:
    // 1. Initialize idx and max_diff with the first event's index and time
    // 2. Iterate over events from the second one, computing the difference in times
    //    between consecutive events
    // 3. Update idx and max_diff accordingly
    fn button_with_longest_time(&self, events: &[Vec<i32>]) -> i32 {
        let mut idx = events[0][0];
        let mut max_diff = events[0][1];
        for i in 1..events.len() {
            let p = &events[i - 1];
            let q = &events[i];
            let d = q[1] - p[1];
            if d > max_diff || (d == max_diff && q[0] < idx) {
                idx = q[0];
                max_diff = d;
            }
        }
        idx
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the number of events (n) from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    // Read n lines of input, each containing 2 integers: [index, time]
    let stdin = io::stdin();
    let mut events = Vec::with_capacity(n);

    for _ in 0..n {
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;

        // Each event is stored as [index, time]
        events.push(vec![nums[0], nums[1]]);
    }

    // Compute the result using the same logic as the C++ solution
    let solution = Solution;
    let result = solution.button_with_longest_time(&events);

    // Output the result
    println!("{}", result);

    Ok(())
}