// Problem: Weekly Contest 428 Problem 1
use std::io;

// Function to calculate the button with the longest press time
fn button_with_longest_time(events: &[[i32; 2]]) -> i32 {
    if events.is_empty() {
        return 0; // Handle empty input if needed, though problem statement implies non-empty input
    }

    let mut longest_time = events[0][1];
    let mut prev_press_time = events[0][1];
    let mut longest_index = events[0][0];

    for i in 1..events.len() {
        let current_press_time = events[i][1];
        let current_time = current_press_time - prev_press_time;

        if current_time > longest_time ||
            (current_time == longest_time && events[i][0] < longest_index) {
            longest_time = current_time;
            longest_index = events[i][0];
        }
        prev_press_time = current_press_time;
    }
    longest_index
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input for n");

    let mut events: Vec<[i32; 2]> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut event_str = String::new();
        io::stdin().read_line(&mut event_str).expect("Failed to read line");
        let event_parts: Vec<i32> = event_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for event value"))
            .collect();
        if event_parts.len() != 2 {
            panic!("Invalid input format for event");
        }
        events.push([event_parts[0], event_parts[1]]);
    }

    let result = button_with_longest_time(&events);
    println!("{}", result);
}