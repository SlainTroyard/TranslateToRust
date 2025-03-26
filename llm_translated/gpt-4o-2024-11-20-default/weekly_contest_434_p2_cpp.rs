use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn count_mentions(number_of_users: usize, mut events: Vec<Vec<String>>) -> Vec<i32> {
        // Helper function for event sorting
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

        // Create online times and counts vectors
        let mut online_times = vec![0; number_of_users];
        let mut counts = vec![0; number_of_users];
        let mut all_count = 0;

        for event in events.iter() {
            let time = event[1].parse::<i32>().unwrap();
            match event[0].as_str() {
                "OFFLINE" => {
                    if let Ok(id) = Solution::extract_id(&event[2]) {
                        if id >= 0 && id < number_of_users as i32 {
                            online_times[id as usize] = time + 60;
                        }
                    }
                }
                "MESSAGE" => {
                    match event[2].as_str() {
                        "ALL" => {
                            all_count += 1;
                        }
                        "HERE" => {
                            for i in 0..number_of_users {
                                if online_times[i] <= time {
                                    counts[i] += 1;
                                }
                            }
                        }
                        mentions => {
                            Solution::update_counts(&mut counts, mentions, number_of_users);
                        }
                    }
                }
                _ => {}
            }
        }

        if all_count > 0 {
            for count in counts.iter_mut() {
                *count += all_count;
            }
        }

        counts
    }

    fn extract_id(id_str: &str) -> Result<i32, String> {
        if id_str.starts_with("id") {
            id_str[2..].parse::<i32>().map_err(|e| format!("Error parsing ID: {}", e))
        } else {
            id_str.parse::<i32>().map_err(|e| format!("Error parsing ID: {}", e))
        }
    }

    fn update_counts(counts: &mut Vec<i32>, mentions: &str, number_of_users: usize) {
        mentions.split_whitespace().for_each(|id_str| {
            if let Ok(id) = Solution::extract_id(id_str) {
                if id >= 0 && id < number_of_users as i32 {
                    counts[id as usize] += 1;
                }
            }
        });
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of users and events
    let first_line = lines.next().unwrap().unwrap();
    let mut split = first_line.split_whitespace();
    let number_of_users = split.next().unwrap().parse::<usize>().unwrap();
    let number_of_events = split.next().unwrap().parse::<usize>().unwrap();

    let mut events = Vec::new();

    // Read events
    for _ in 0..number_of_events {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let event_type = parts.next().unwrap().to_string();
        let time = parts.next().unwrap().to_string();

        if event_type == "MESSAGE" {
            let message_type = parts.next().unwrap().to_string();
            if message_type == "ALL" || message_type == "HERE" {
                events.push(vec![event_type, time, message_type]);
            } else {
                // Handle mentions case
                let mut mentions = String::new();
                mentions.push_str(&message_type);
                mentions.push_str(" ");
                mentions.push_str(&parts.collect::<Vec<&str>>().join(" "));
                mentions = mentions.trim().to_string();
                events.push(vec![event_type, time, mentions]);
            }
        } else if event_type == "OFFLINE" {
            // Handle OFFLINE event
            let user_id = parts.next().unwrap().to_string();
            events.push(vec![event_type, time, user_id]);
        }
    }

    // Process and output result
    let solution = Solution;
    let result = solution.count_mentions(number_of_users, events);

    for count in result {
        print!("{} ", count);
    }
    println!();
}