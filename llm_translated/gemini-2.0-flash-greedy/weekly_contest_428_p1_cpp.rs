use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn button_with_longest_time(&self, events: Vec<Vec<i32>>) -> i32 {
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

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Invalid input for n");

    let mut events: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let values: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for event values"))
            .collect();
        events.push(vec![values[0], values[1]]);
    }

    let solution = Solution {};
    let result = solution.button_with_longest_time(events);
    println!("{}", result);
}