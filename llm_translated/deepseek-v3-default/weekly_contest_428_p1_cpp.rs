use std::io;

// Define a struct to encapsulate the solution
struct Solution;

impl Solution {
    // Function to find the button with the longest time difference
    fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
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
    // Read the number of events from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    // Read the events from stdin
    let mut events: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let event: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        events.push(event);
    }

    // Create an instance of the Solution struct
    let solution = Solution;

    // Call the function and print the result
    let result = solution.button_with_longest_time(events);
    println!("{}", result);
}