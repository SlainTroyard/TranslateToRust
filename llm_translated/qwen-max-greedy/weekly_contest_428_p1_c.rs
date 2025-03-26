use std::io;

fn button_with_longest_time(events: &[(usize, usize)]) -> usize {
    let mut longest_time = events[0].1;
    let mut prev = events[0].1;
    let mut longest_index = events[0].0;

    for i in 1..events.len() {
        let current_press = events[i].1;
        let current_time = current_press - prev;

        if current_time > longest_time ||
           (current_time == longest_time && events[i].0 < longest_index) {
            longest_time = current_time;
            longest_index = events[i].0;
        }
        prev = current_press;
    }
    longest_index
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    let mut events = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let values: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse()?)
            .collect();
        if values.len() != 2 {
            eprintln!("Invalid input format");
            return Err("Invalid input format".into());
        }
        events.push((values[0], values[1]));
    }

    let result = button_with_longest_time(&events);
    println!("{}", result);

    Ok(())
}