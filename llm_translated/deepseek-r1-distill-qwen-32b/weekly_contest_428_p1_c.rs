use std::io;

struct Event {
    button: i32,
    time: i32,
}

fn button_with_longest_time(events: &[Event]) -> i32 {
    let mut longest_time = events[0].time;
    let mut prev_time = events[0].time;
    let mut longest_button = events[0].button;

    for i in 1..events.len() {
        let current_time = events[i].time - prev_time;
        if current_time > longest_time || (current_time == longest_time && events[i].button < longest_button) {
            longest_time = current_time;
            longest_button = events[i].button;
        }
        prev_time = events[i].time;
    }

    longest_button
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        return Ok(());
    }

    let n = tokens[0].parse::<i32>()?;
    let mut events = Vec::with_capacity(n as usize);

    for i in 0..n {
        let button = tokens[1 + 2 * i].parse::<i32>()?;
        let time = tokens[2 + 2 * i].parse::<i32>()?;
        events.push(Event { button, time });
    }

    let result = button_with_longest_time(&events);
    println!("{}", result);

    Ok(())
}