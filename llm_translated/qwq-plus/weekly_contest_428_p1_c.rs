use std::io;

/// Calculates the button with the longest press time based on the given events.
///
/// # Arguments
/// * `events` - A vector of events, each containing (button index, time).
///
/// # Returns
/// The index of the button with the longest press time. In case of a tie, the smallest index is chosen.
fn button_with_longest_time(events: &Vec<(i32, i32)>) -> i32 {
    if events.is_empty() {
        panic!("No events provided");
    }

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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().expect("Invalid number of events");

    let mut events = Vec::with_capacity(n);
    for _ in 0..n {
        let button: i32 = tokens.next().unwrap().parse().expect("Invalid button index");
        let time: i32 = tokens.next().unwrap().parse().expect("Invalid time");
        events.push((button, time));
    }

    let result = button_with_longest_time(&events);
    println!("{}", result);
}