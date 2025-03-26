use std::io;
use std::io::Read;

// Function to calculate the button with the longest press time
fn button_with_longest_time(events: &Vec<Vec<i32>>) -> i32 {
    let mut longest_time = events[0][1];
    let mut prev = events[0][1];
    let mut longest_index = events[0][0];

    for i in 1..events.len() {
        let current_press = events[i][1];
        let current_time = current_press - prev;

        if current_time > longest_time ||
           (current_time == longest_time && events[i][0] < longest_index) {
            longest_time = current_time;
            longest_index = events[i][0];
        }
        prev = current_press;
    }
    longest_index
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut events: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut nums = line.split_whitespace();
        let a: i32 = nums.next().unwrap().parse().unwrap();
        let b: i32 = nums.next().unwrap().parse().unwrap();
        events.push(vec![a, b]);
    }

    let result = button_with_longest_time(&events);
    println!("{}", result);

    Ok(())
}