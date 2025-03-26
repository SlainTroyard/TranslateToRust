use std::io;

fn main() {
    // Read the number of events
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    // Read the events
    let mut events = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let parts: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        events.push((parts[0], parts[1]));
    }

    // Find the button with the longest time
    let result = button_with_longest_time(&events);

    // Output the result
    println!("{}", result);
}

fn button_with_longest_time(events: &[(i32, i32)]) -> i32 {
    let mut idx = events[0].0;
    let mut max_diff = events[0].1;

    for i in 1..events.len() {
        let p = events[i - 1];
        let q = events[i];
        let d = q.1 - p.1;

        if d > max_diff || (d == max_diff && q.0 < idx) {
            idx = q.0;
            max_diff = d;
        }
    }

    idx
}