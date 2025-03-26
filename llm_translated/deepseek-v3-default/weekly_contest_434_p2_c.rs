use std::collections::HashMap;
use std::io::{self, BufRead};
use std::str::FromStr;

// Convert a string to a number by extracting digits
fn str_to_num(s: &str) -> i32 {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |acc, c| acc * 10 + (c as i32 - '0' as i32))
}

// Extract the numeric part from an ID string (e.g., "id123" -> 123)
fn extract_id_number(id_str: &str) -> Option<i32> {
    id_str
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}

fn count_mentions(number_of_users: usize, events: &[Vec<String>]) -> Vec<i32> {
    let mut time_arr: Vec<i32> = events.iter().map(|e| str_to_num(&e[1])).collect();
    let mut order: Vec<usize> = (0..events.len()).collect();

    // Sort events by time and type
    for i in (1..events.len()).rev() {
        for j in 0..i {
            if time_arr[order[j + 1]] < time_arr[order[j]]
                || (time_arr[order[j + 1]] == time_arr[order[j]] && events[order[j + 1]][0] == "O")
            {
                order.swap(j, j + 1);
            }
        }
    }

    let mut online: Vec<i32> = vec![0; number_of_users];
    let mut mention: Vec<i32> = vec![0; number_of_users];

    for &i in &order {
        let event = &events[i];
        match event[0].as_str() {
            "M" => match event[2].as_str() {
                "A" => mention.iter_mut().for_each(|m| *m += 1),
                "H" => {
                    let time = str_to_num(&event[1]);
                    for j in 0..number_of_users {
                        if online[j] == 0 {
                            mention[j] += 1;
                        } else if online[j] <= time {
                            online[j] = 0;
                            mention[j] += 1;
                        }
                    }
                }
                _ => {
                    let ids: Vec<&str> = event[2].split(' ').collect();
                    for id in ids {
                        if let Some(user_id) = extract_id_number(id) {
                            if user_id < number_of_users as i32 {
                                mention[user_id as usize] += 1;
                            }
                        }
                    }
                }
            },
            "O" => {
                if let Some(user_id) = extract_id_number(&event[2]) {
                    if user_id < number_of_users as i32 {
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

    // Read the number of users and events
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let number_of_users: usize = parts.next().unwrap().parse().unwrap();
    let events_size: usize = parts.next().unwrap().parse().unwrap();

    // Read the events
    let mut events: Vec<Vec<String>> = Vec::with_capacity(events_size);
    for _ in 0..events_size {
        let line = lines.next().unwrap().unwrap();
        let event: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        events.push(event);
    }

    // Compute mentions
    let result = count_mentions(number_of_users, &events);

    // Print the result
    print!("Mentions: ");
    for count in result {
        print!("{} ", count);
    }
    println!();
}