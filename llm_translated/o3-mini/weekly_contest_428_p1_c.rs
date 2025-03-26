use std::io::{self, Read};

/// Computes the button with the longest press time
///
/// # Arguments
///
/// * `events` - A slice of (button_index, press_time) tuples.
///
/// # Returns
///
/// The button index that had the longest press time.
/// If there is a tie, the button with the smaller index is returned.
fn button_with_longest_time(events: &[(i32, i32)]) -> i32 {
    // Initialize with the first event.
    let mut longest_time = events[0].1;
    let mut prev = events[0].1;
    let mut longest_index = events[0].0;

    // Iterate through the remaining events.
    for &(button, press_time) in events.iter().skip(1) {
        let current_press = press_time;
        let current_time = current_press - prev;

        // Check if the current press time difference is greater,
        // or equal with a lower button index.
        if current_time > longest_time || (current_time == longest_time && button < longest_index) {
            longest_time = current_time;
            longest_index = button;
        }
        // Update previous press time.
        prev = current_press;
    }
    longest_index
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read entire input into a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input by whitespace and create an iterator over tokens.
    let mut tokens = input.split_whitespace();

    // The first token is the number of events.
    let n: usize = tokens
        .next()
        .ok_or("Expected a number for events count")?
        .parse()?;

    // Read the events into a vector.
    // Each event consists of two integers: button index and press time.
    let mut events: Vec<(i32, i32)> = Vec::with_capacity(n);
    for _ in 0..n {
        let button: i32 = tokens
            .next()
            .ok_or("Expected a button value for an event")?
            .parse()?;
        let press_time: i32 = tokens
            .next()
            .ok_or("Expected a press time for an event")?
            .parse()?;
        events.push((button, press_time));
    }

    // Call the function to compute the result.
    let result = button_with_longest_time(&events);

    // Print the result (exact same output format as the original C code).
    println!("{}", result);

    Ok(())
}