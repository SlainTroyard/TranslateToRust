use std::io::{self, BufRead};
use std::cmp;

fn button_with_longest_time(events: &Vec<(i32, i32)>) -> i32 {
    let mut longest_time = events[0].1; // The first time is the default longest time
    let mut prev = events[0].1;        // Track the previous button press time
    let mut longest_index = events[0].0; // The button ID of the longest press time

    for i in 1..events.len() {
        let current_press = events[i].1; // Current button press time
        let current_time = current_press - prev; // Calculate time held

        // Update the longest press logic
        if current_time > longest_time ||
            (current_time == longest_time && events[i].0 < longest_index) {
            longest_time = current_time; // Update longest time
            longest_index = events[i].0; // Update button ID
        }

        prev = current_press; // Update previous press time
    }

    longest_index
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of events
    let n: usize = lines.next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Expected an integer for n");

    // Read the event data into a Vec
    let mut events: Vec<(i32, i32)> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let button_id: i32 = parts.next()
            .unwrap()
            .parse()
            .expect("Expected an integer for button ID");
        let timestamp: i32 = parts.next()
            .unwrap()
            .parse()
            .expect("Expected an integer for timestamp");
        
        events.push((button_id, timestamp));
    }

    // Call the function to calculate the button with the longest press time
    let result = button_with_longest_time(&events);

    // Output the result
    println!("{}", result);
}