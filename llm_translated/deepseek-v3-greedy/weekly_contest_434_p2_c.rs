use std::io::{self, BufRead};
use std::collections::HashMap;

// Convert a string to a number
fn str_to_num(s: &str) -> i32 {
    s.chars()
        .filter(|c| c.is_digit(10))
        .fold(0, |acc, c| acc * 10 + (c as i32 - '0' as i32))
}

// Extract the numeric part from an ID string (e.g., "id123" -> 123)
fn extract_id_number(id_str: &str) -> i32 {
    let num_start = id_str.find(|c: char| c.is_digit(10)).unwrap_or(id_str.len());
    if num_start == id_str.len() {
        return -1;
    }
    str_to_num(&id_str[num_start..])
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
                "A" => {
                    for j in 0..number_of_users {
                        mention[j] += 1;
                    }
                }
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
                        let user_id = extract_id_number(id);
                        if user_id >= 0 && (user_id as usize) < number_of_users {
                            mention[user_id as usize] += 1;
                        }
                    }
                }
            },
            "O" => {
                let user_id = extract_id_number(&event[2]);
                if user_id >= 0 && (user_id as usize) < number_of_users {
                    online[user_id as usize] = str_to_num(&event[1]) + 60;
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

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let number_of_users: usize = parts.next().unwrap().parse().unwrap();
    let events_size: usize = parts.next().unwrap().parse().unwrap();

    let mut events: Vec<Vec<String>> = Vec::with_capacity(events_size);
    for _ in 0..events_size {
        let line = lines.next().unwrap().unwrap();
        let event: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        events.push(event);
    }

    let result = count_mentions(number_of_users, &events);

    print!("Mentions: ");
    for &count in &result {
        print!("{} ", count);
    }
    println!();
}