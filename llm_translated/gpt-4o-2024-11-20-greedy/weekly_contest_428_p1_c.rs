use std::io::{self, BufRead};

/// Function to calculate the button with the longest press time
fn button_with_longest_time(events: &Vec<(i32, i32)>) -> i32 {
    let mut longest_time = events[0].1;
    let mut prev = events[0].1;
    let mut longest_index = events[0].0;

    for i in 1..events.len() {
        let current_press = events[i].1;
        let current_time = current_press - prev;

        if current_time > longest_time || (current_time == longest_time && events[i].0 < longest_index) {
            longest_time = current_time;
            longest_index = events[i].0;
        }
        prev = current_press;
    }

    longest_index
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of events
    let n: usize = lines.next()
        .expect("Expected number of events")
        .expect("Failed to read input")
        .trim()
        .parse()
        .expect("Failed to parse number of events");

    // Read the events
    let mut events = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next()
            .expect("Expected event data")
            .expect("Failed to read input");
        let mut parts = line.split_whitespace();
        let button: i32 = parts.next()
            .expect("Expected button ID")
            .parse()
            .expect("Failed to parse button ID");
        let time: i32 = parts.next()
            .expect("Expected press time")
            .parse()
            .expect("Failed to parse press time");
        events.push((button, time));
    }

    // Calculate the result
    let result = button_with_longest_time(&events);

    // Print the result
    println!("{}", result);
}