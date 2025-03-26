use std::io::{self, BufRead};
use std::collections::HashMap;

fn str_to_num(s: &str) -> i32 {
    s.chars().filter(|c| c.is_digit(10)).fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap() as i32)
}

/// Extracts the numeric part from the ID string (e.g., "id123" -> 123).
fn extract_id_number(id_str: &str) -> Option<i32> {
    let num_start = id_str.chars().find(|c| c.is_digit(10))?;
    Some(str_to_num(&id_str[id_str.find(num_start).unwrap()..]))
}

/// Main function that processes the events and calculates mentions
fn count_mentions(
    number_of_users: usize,
    events: Vec<Vec<String>>,
) -> Vec<i32> {
    // Parse event times and initialize order to sort events
    let mut time_arr: Vec<i32> = events.iter().map(|event| str_to_num(&event[1])).collect();
    let mut order: Vec<usize> = (0..events.len()).collect();

    // Sort events by time and type (consider "O" events with higher priority for ties)
    order.sort_by(|&i, &j| {
        let time_cmp = time_arr[i].cmp(&time_arr[j]);
        if time_cmp == std::cmp::Ordering::Equal {
            events[j][0].cmp(&events[i][0]) // "O" comes before "M"
        } else {
            time_cmp
        }
    });

    // Initialize online and mention counts
    let mut online = vec![0; number_of_users];
    let mut mention = vec![0; number_of_users];

    for &idx in &order {
        let event = &events[idx];
        match event[0].as_bytes()[0] as char {
            'M' => {
                if event[2] == "ALL" {
                    for mention_count in mention.iter_mut() {
                        *mention_count += 1;
                    }
                } else if event[2] == "HERE" {
                    let time = str_to_num(&event[1]);
                    for j in 0..number_of_users {
                        if online[j] == 0 {
                            mention[j] += 1;
                        } else if online[j] <= time {
                            online[j] = 0; // Mark offline
                            mention[j] += 1;
                        }
                    }
                } else {
                    // Break down space-separated IDs and increment specific mentions
                    for id_str in event[2].split_whitespace() {
                        if let Some(user_id) = extract_id_number(id_str) {
                            if user_id >= 0 && (user_id as usize) < number_of_users {
                                mention[user_id as usize] += 1;
                            }
                        }
                    }
                }
            }
            'O' => {
                if let Some(user_id) = extract_id_number(&event[2]) {
                    if user_id >= 0 && (user_id as usize) < number_of_users {
                        online[user_id as usize] = str_to_num(&event[1]) + 60;
                    }
                }
            }
            _ => {}
        }
    }

    mention
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let header = lines.next().unwrap().unwrap();
    let mut header_split = header.split_whitespace();

    let number_of_users: usize = header_split.next().unwrap().parse().unwrap();
    let events_size: usize = header_split.next().unwrap().parse().unwrap();

    let mut events: Vec<Vec<String>> = Vec::with_capacity(events_size);

    for _ in 0..events_size {
        if let Some(Ok(line)) = lines.next() {
            let parts: Vec<String> = line.split_whitespace().map(String::from).collect();
            events.push(parts);
        }
    }

    // Process events and compute mentions
    let result = count_mentions(number_of_users, events);

    // Print the output
    print!("Mentions: ");
    for (i, value) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", value);
    }
    println!();
}