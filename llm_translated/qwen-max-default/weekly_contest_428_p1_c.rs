use std::io;

// Function to calculate the button with the longest press time
fn button_with_longest_time(events: &[(i32, i32)]) -> i32 {
    let mut longest_time = events[0].1;
    let mut prev = events[0].1;
    let mut longest_index = events[0].0;

    for (i, &event) in events.iter().enumerate().skip(1) {
        let current_press = event.1;
        let current_time = current_press - prev;

        if current_time > longest_time || (current_time == longest_time && event.0 < longest_index) {
            longest_time = current_time;
            longest_index = event.0;
        }
        prev = current_press;
    }
    longest_index
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the number of events
    let mut n = String::new();
    io::stdin().read_line(&mut n)?;
    let n: usize = n.trim().parse()?;

    // Read the events
    let mut events: Vec<(i32, i32)> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let mut iter = line.trim().split_whitespace();
        let button: i32 = iter.next().ok_or("Invalid input")?.parse()?;
        let press: i32 = iter.next().ok_or("Invalid input")?.parse()?;
        events.push((button, press));
    }

    // Calculate the button with the longest press time
    let result = button_with_longest_time(&events);

    // Print the result
    println!("{}", result);

    Ok(())
}