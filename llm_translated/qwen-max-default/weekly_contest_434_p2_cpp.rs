use std::io::{self, BufRead, Write};
use std::str::FromStr;

fn main() {
    let (number_of_users, number_of_events) = read_input_numbers();
    let mut events = read_events(number_of_events);

    let solution = Solution {};
    let result = solution.count_mentions(number_of_users, &mut events);

    for count in result {
        print!("{} ", count);
    }
    println!();
}

fn read_input_numbers() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (nums[0], nums[1])
}

fn read_events(number_of_events: usize) -> Vec<Vec<String>> {
    let stdin = io::stdin();
    let mut events = vec![vec![String::new(); 3]; number_of_events];

    for i in 0..number_of_events {
        let mut event_type = String::new();
        let mut time = String::new();
        stdin.lock().read_line(&mut event_type).unwrap();
        stdin.lock().read_line(&mut time).unwrap();

        events[i][0] = event_type.trim().to_string();
        events[i][1] = time.trim().to_string();

        if events[i][0] == "MESSAGE" {
            let mut message_type = String::new();
            stdin.lock().read_line(&mut message_type).unwrap();
            events[i][2] = message_type.trim().to_string();

            if events[i][2] != "ALL" && events[i][2] != "HERE" {
                let mut rest_of_line = String::new();
                stdin.lock().read_line(&mut rest_of_line).unwrap();
                events[i][2].push_str(rest_of_line.trim());
            }
        } else if events[i][0] == "OFFLINE" {
            let mut id = String::new();
            stdin.lock().read_line(&mut id).unwrap();
            events[i][2] = id.trim().to_string();
        }
    }

    events
}

struct Solution;

impl Solution {
    fn count_mentions(&self, number_of_users: usize, events: &mut Vec<Vec<String>>) -> Vec<i32> {
        events.sort_by(|a, b| cmp(a, b));
        let mut online_times = vec![0; number_of_users];
        let mut counts = vec![0; number_of_users];
        let mut all_count = 0;

        for event in events.iter_mut() {
            let time = i32::from_str(&event[1]).unwrap();
            if event[0] == "OFFLINE" {
                if let Ok(id) = extract_id(&event[2]) {
                    if id < number_of_users {
                        online_times[id] = time + 60;
                    }
                }
                continue;
            }

            let mentions = &event[2];
            if mentions == "ALL" {
                all_count += 1;
            } else if mentions == "HERE" {
                for (i, &online_time) in online_times.iter().enumerate() {
                    if online_time <= time {
                        counts[i] += 1;
                    }
                }
            } else {
                update_counts(&mut counts, mentions, number_of_users);
            }
        }

        if all_count > 0 {
            for count in counts.iter_mut() {
                *count += all_count;
            }
        }

        counts
    }
}

fn cmp(a: &Vec<String>, b: &Vec<String>) -> std::cmp::Ordering {
    let time_a = i32::from_str(&a[1]).unwrap();
    let time_b = i32::from_str(&b[1]).unwrap();
    if time_a == time_b && a[0] != b[0] {
        if a[0] == "OFFLINE" { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
    } else {
        time_a.cmp(&time_b)
    }
}

fn extract_id(id_str: &str) -> Result<usize, std::num::ParseIntError> {
    if id_str.starts_with("id") {
        id_str[2..].parse::<usize>()
    } else {
        id_str.parse::<usize>()
    }
}

fn update_counts(counts: &mut [i32], mentions: &str, number_of_users: usize) {
    for id_str in mentions.split_whitespace() {
        if let Ok(id) = extract_id(id_str) {
            if id < number_of_users {
                counts[id] += 1;
            }
        }
    }
}