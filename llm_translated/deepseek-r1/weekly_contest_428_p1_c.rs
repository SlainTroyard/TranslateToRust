use std::io;

fn main() {
    // Read number of events
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Read each event as a pair of integers [button, timestamp]
    let mut events = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        let button: i32 = parts.next().unwrap().parse().unwrap();
        let time: i32 = parts.next().unwrap().parse().unwrap();
        events.push([button, time]);
    }

    // Create column size array (unused but preserved for translation accuracy)
    let events_col_size = vec![2; n];

    // Calculate and print the result
    println!("{}", button_with_longest_time(&events, &events_col_size));
}

fn button_with_longest_time(events: &[[i32; 2]], _events_col_size: &[usize]) -> i32 {
    // Initialize with first event's data
    let mut longest_time = events[0][1];
    let mut prev_time = events[0][1];
    let mut longest_index = events[0][0];

    // Iterate through subsequent events starting from index 1
    for event in &events[1..] {
        let current_time = event[1] - prev_time;
        
        // Update longest duration if current is longer, or same duration with smaller button index
        if current_time > longest_time || (current_time == longest_time && event[0] < longest_index) {
            longest_time = current_time;
            longest_index = event[0];
        }
        
        prev_time = event[1];
    }

    longest_index
}