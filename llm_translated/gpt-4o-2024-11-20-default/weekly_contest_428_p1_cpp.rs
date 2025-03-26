// Problem: Weekly Contest 428 Problem 1

use std::io;

struct Solution;

impl Solution {
    // Function to find the button with the longest time
    pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        let mut idx = events[0][0]; // Initial index
        let mut max_diff = events[0][1]; // Initial time interval

        for i in 1..events.len() {
            let p = &events[i - 1]; // Previous event
            let q = &events[i]; // Current event
            let d = q[1] - p[1]; // Compute the time difference

            // Update `idx` and `max_diff` if a greater difference is found
            // Or if the difference is the same but the button index is smaller
            if d > max_diff || (d == max_diff && q[0] < idx) {
                idx = q[0];
                max_diff = d;
            }
        }
        idx
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    // Read the number of events
    stdin.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut events = Vec::new();
    input.clear();

    // Read the events
    for _ in 0..n {
        stdin.read_line(&mut input).unwrap();
        let event: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        events.push(event);
        input.clear();
    }

    // Use the solution
    let solution = Solution;
    let result = solution.button_with_longest_time(events);

    // Print the result
    println!("{}", result);
}