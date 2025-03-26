use std::cmp::Ordering;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn count_mentions(number_of_users: usize, mut events: Vec<Vec<String>>) -> Vec<i32> {
        // Sort events based on the custom comparator
        events.sort_by(|a, b| {
            let time_a: i32 = a[1].parse().unwrap();
            let time_b: i32 = b[1].parse().unwrap();

            if time_a == time_b {
                // Prioritize OFFLINE events when times are equal but types differ
                if a[0] != b[0] {
                    match (a[0].as_str(), b[0].as_str()) {
                        ("OFFLINE", _) => Ordering::Less,
                        (_, "OFFLINE") => Ordering::Greater,
                        _ => Ordering::Greater, // Original behavior for non-OFFLINE different types
                    }
                } else {
                    Ordering::Equal
                }
            } else {
                time_a.cmp(&time_b)
            }
        });

        let mut online_times = vec![0; number_of_users];
        let mut counts = vec![0; number_of_users];
        let mut all_count = 0;

        for event in &events {
            let event_type = event[0].as_str();
            let time: i32 = event[1].parse().unwrap();

            match event_type {
                "OFFLINE" => {
                    let id_str = &event[2];
                    let id = extract_id(id_str);
                    if id < number_of_users {
                        online_times[id] = time + 60;
                    }
                }
                "MESSAGE" => {
                    let mentions = &event[2];
                    match mentions.as_str() {
                        "ALL" => all_count += 1,
                        "HERE" => {
                            for (i, &online_time) in online_times.iter().enumerate() {
                                if online_time <= time {
                                    counts[i] += 1;
                                }
                            }
                        }
                        _ => update_counts(&mut counts, mentions),
                    }
                }
                _ => {}
            }
        }

        if all_count > 0 {
            for count in &mut counts {
                *count += all_count;
            }
        }

        counts
    }
}

fn extract_id(id_str: &str) -> usize {
    let s = if id_str.starts_with("id") {
        &id_str[2..]
    } else {
        id_str
    };
    s.parse().unwrap_or(0)
}

fn update_counts(counts: &mut [i32], mentions: &str) {
    for part in mentions.split_whitespace() {
        let id = extract_id(part);
        if id < counts.len() {
            counts[id] += 1;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    
    // Read first line for user and event counts
    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let number_of_users: usize = parts.next().unwrap().parse().unwrap();
    let number_of_events: usize = parts.next().unwrap().parse().unwrap();

    let mut events = Vec::with_capacity(number_of_events);

    // Process each event line
    for _ in 0..number_of_events {
        let line = lines.next().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        let event_type = parts[0].to_string();
        let time = parts[1].to_string();
        let third_part = if event_type == "MESSAGE" {
            parts.get(2..).map(|p| p.join(" ")).unwrap_or_default()
        } else if event_type == "OFFLINE" {
            parts.get(2).map(|s| s.to_string()).unwrap_or_default()
        } else {
            String::new()
        };

        events.push(vec![event_type, time, third_part]);
    }

    let result = Solution::count_mentions(number_of_users, events);

    // Print results in space-separated format
    for count in result {
        print!("{} ", count);
    }
    println!();
}