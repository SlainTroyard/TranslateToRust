use std::io::{self, BufRead};
use std::str::FromStr;
use std::collections::HashMap;

// Helper function to convert a string to a number
fn str_to_num(s: &str) -> i32 {
    s.chars()
        .filter(|c| c.is_digit(10))
        .fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap() as i32)
}

// Extract the numeric part of an ID string (e.g., "id123" -> 123)
fn extract_id_number(id_str: &str) -> Option<i32> {
    id_str.chars()
        .skip_while(|c| !c.is_digit(10))
        .collect::<String>()
        .parse::<i32>()
        .ok()
}

// Main function to count mentions
fn count_mentions(
    number_of_users: usize,
    events: Vec<Vec<String>>,
) -> Vec<i32> {
    let mut time_arr: Vec<i32> = events.iter().map(|event| str_to_num(&event[1])).collect();
    let mut order: Vec<usize> = (0..events.len()).collect();

    // Sort events based on time and type
    order.sort_by(|&a, &b| {
        let time_a = time_arr[a];
        let time_b = time_arr[b];
        if time_a != time_b {
            time_a.cmp(&time_b)
        } else {
            events[b][0].cmp(&events[a][0]) // 'O' comes before 'M'
        }
    });

    let mut online = vec![0; number_of_users];
    let mut mention = vec![0; number_of_users];

    for &i in &order {
        let event = &events[i];
        match event[0].as_str() {
            "M" => {
                if event[2] == "A" {
                    for j in 0..number_of_users {
                        mention[j] += 1;
                    }
                } else if event[2] == "H" {
                    let time = str_to_num(&event[1]);
                    for j in 0..number_of_users {
                        if online[j] == 0 {
                            mention[j] += 1;
                        } else if online[j] <= time {
                            online[j] = 0;
                            mention[j] += 1;
                        }
                    }
                } else {
                    let mut ids = event[2].split_whitespace();
                    while let Some(id) = ids.next() {
                        if let Some(user_id) = extract_id_number(id) {
                            if (0..number_of_users as i32).contains(&user_id) {
                                mention[user_id as usize] += 1;
                            }
                        }
                    }
                }
            }
            "O" => {
                if let Some(user_id) = extract_id_number(&event[2]) {
                    if (0..number_of_users as i32).contains(&user_id) {
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of users and events
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_split = first_line.split_whitespace();
    let number_of_users: usize = first_line_split.next().unwrap().parse().unwrap();
    let events_size: usize = first_line_split.next().unwrap().parse().unwrap();

    // Read events
    let mut events = Vec::new();
    for _ in 0..events_size {
        let mut event = Vec::new();
        for _ in 0..3 {
            let line = lines.next().unwrap().unwrap();
            event.push(line);
        }
        events.push(event);
    }

    // Process and get mentions
    let mentions = count_mentions(number_of_users, events);

    // Print results
    println!("Mentions: {}", mentions.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(" "));
}