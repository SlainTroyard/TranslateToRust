use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp::Ordering;

fn extract_id(id_str: &str) -> i32 {
    if id_str.starts_with("id") {
        id_str[2..].parse().unwrap_or_else(|_| {
            eprintln!("Error parsing ID: {}", id_str);
            0
        })
    } else {
        id_str.parse().unwrap_or_else(|_| {
            eprintln!("Error parsing ID: {}", id_str);
            0
        })
    }
}

fn update_counts(counts: &mut Vec<i32>, mentions: &str) {
    for id_str in mentions.split_whitespace() {
        let id = extract_id(id_str);
        if id >= 0 && id < counts.len() as i32 {
            counts[id as usize] += 1;
        }
    }
}

fn count_mentions(number_of_users: i32, events: &mut Vec<Vec<String>>) -> Vec<i32> {
    events.sort_by(|a, b| {
        let time_a = a[1].parse::<i32>().unwrap();
        let time_b = b[1].parse::<i32>().unwrap();
        if time_a == time_b && a[0] != b[0] {
            if a[0] == "OFFLINE" {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            time_a.cmp(&time_b)
        }
    });

    let mut online_times = vec![0; number_of_users as usize];
    let mut counts = vec![0; number_of_users as usize];
    let mut all_count = 0;

    for event in events {
        let time = event[1].parse::<i32>().unwrap();
        if event[0] == "OFFLINE" {
            let id = extract_id(&event[2]);
            if id >= 0 && id < number_of_users {
                online_times[id as usize] = time + 60;
            }
            continue;
        }

        let mentions = &event[2];
        if mentions == "ALL" {
            all_count += 1;
        } else if mentions == "HERE" {
            for i in 0..number_of_users as usize {
                if online_times[i] <= time {
                    counts[i] += 1;
                }
            }
        } else {
            update_counts(&mut counts, mentions);
        }
    }

    if all_count > 0 {
        for count in &mut counts {
            *count += all_count;
        }
    }

    counts
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let number_of_users = parts.next().unwrap().parse::<i32>().unwrap();
    let number_of_events = parts.next().unwrap().parse::<i32>().unwrap();

    let mut events = Vec::with_capacity(number_of_events as usize);
    for _ in 0..number_of_events {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let event_type = parts.next().unwrap().to_string();
        let time = parts.next().unwrap().to_string();
        let mut third_arg = String::new();

        if event_type == "MESSAGE" {
            let message_type = parts.next().unwrap().to_string();
            if message_type == "ALL" || message_type == "HERE" {
                third_arg = message_type;
            } else {
                third_arg = message_type;
                if let Some(rest) = parts.next() {
                    third_arg.push(' ');
                    third_arg.push_str(rest);
                }
            }
        } else if event_type == "OFFLINE" {
            third_arg = parts.next().unwrap().to_string();
        }

        events.push(vec![event_type, time, third_arg]);
    }

    let result = count_mentions(number_of_users, &mut events);
    for count in result {
        print!("{} ", count);
    }
    println!();
}