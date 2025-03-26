use std::io;

fn main() {
    // Read the number of events
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid n");

    // Read each event (index, timestamp)
    let mut events = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid integer"))
            .collect();
        events.push(vec![parts[0], parts[1]]);
    }

    // Calculate and print the result
    let result = button_with_longest_time(&events);
    println!("{}", result);
}

/// Finds the button index with the maximum consecutive time difference.
/// Preserves original algorithm logic from C++ implementation.
fn button_with_longest_time(events: &[Vec<i32>]) -> i32 {
    // Initialize with first event's index and timestamp
    let mut idx = events[0][0];
    let mut max_diff = events[0][1];

    // Iterate through consecutive event pairs
    for i in 1..events.len() {
        let p = &events[i - 1];
        let q = &events[i];
        let d = q[1] - p[1];

        // Update maximum difference and index according to problem rules
        if d > max_diff || (d == max_diff && q[0] < idx) {
            idx = q[0];
            max_diff = d;
        }
    }

    idx
}