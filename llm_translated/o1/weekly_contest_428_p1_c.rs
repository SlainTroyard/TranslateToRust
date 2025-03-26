// Weekly Contest 428 Problem 1
// Translated from C to Rust

use std::error::Error;
use std::io::{self, BufRead};

// Function to calculate the button with the longest press time
fn button_with_longest_time(events: &[(i32, i32)]) -> i32 {
    // Initialize variables based on the first event
    let mut longest_time = events[0].1;
    let mut prev = events[0].1;
    let mut longest_index = events[0].0;

    // Iterate through the remaining events
    for i in 1..events.len() {
        let current_press = events[i].1;
        let current_time = current_press - prev;

        // Update the longest press time and index
        if current_time > longest_time
            || (current_time == longest_time && events[i].0 < longest_index)
        {
            longest_time = current_time;
            longest_index = events[i].0;
        }
        prev = current_press;
    }

    longest_index
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the number of events from stdin
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let n = line.trim().parse::<usize>()?;

    // Read each event (button index, time)
    let mut events = Vec::with_capacity(n);
    for _ in 0..n {
        line.clear();
        stdin.lock().read_line(&mut line)?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let button = parts[0].parse::<i32>()?;
        let time = parts[1].parse::<i32>()?;
        events.push((button, time));
    }

    // Compute the result using the translated function
    let result = button_with_longest_time(&events);

    // Print the result to stdout
    println!("{}", result);

    Ok(())
}