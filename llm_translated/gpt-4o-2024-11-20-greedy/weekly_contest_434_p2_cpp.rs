use std::collections::HashMap;
use std::io::{self, BufRead};
use std::str::FromStr;

struct Solution;

impl Solution {
    fn cmp(a: &Vec<String>, b: &Vec<String>) -> std::cmp::Ordering {
        let time_a = a[1].parse::<i32>().unwrap_or(0);
        let time_b = b[1].parse::<i32>().unwrap_or(0);

        if time_a == time_b && a[0] != b[0] {
            if a[0] == "OFFLINE" {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        } else {
            time_a.cmp(&time_b)
        }
    }

    fn extract_id(id_str: &str) -> i32 {
        if id_str.starts_with("id") {
            id_str[2..].parse::<i32>().unwrap_or_else(|_| {
                eprintln!("Error parsing ID: {}", id_str);
                0
            })
        } else {
            id_str.parse::<i32>().unwrap_or_else(|_| {
                eprintln!("Error parsing ID: {}", id_str);
                0
            })
        }
    }

    fn update_counts(counts: &mut Vec<i32>, mentions: &str) {
        for id_str in mentions.split_whitespace() {
            if let Ok(id) = Self::extract_id(id_str) {
                if id >= 0 && (id as usize) < counts.len() {
                    counts[id as usize] += 1;
                }
            }
        }
    }

    pub fn count_mentions(
        number_of_users: usize,
        mut events: Vec<Vec<String>>,
    ) -> Vec<i32> {
        events.sort_by(Self::cmp);

        let mut online_times = vec![0; number_of_users];
        let mut counts = vec![0; number_of_users];
        let mut all_count = 0;

        for event in events {
            let time = event[1].parse::<i32>().unwrap_or(0);

            if event[0] == "OFFLINE" {
                if let Ok(id) = Self::extract_id(&event[2]) {
                    if id >= 0 && (id as usize) < number_of_users {
                        online_times[id as usize] = time + 60;
                    }
                }
                continue;
            }

            let mentions = &event[2];
            if mentions == "ALL" {
                all_count += 1;
            } else if mentions == "HERE" {
                for i in 0..number_of_users {
                    if online_times[i] <= time {
                        counts[i] += 1;
                    }
                }
            } else {
                Self::update_counts(&mut counts, mentions);
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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_split = first_line.split_whitespace();
    let number_of_users: usize = first_line_split.next().unwrap().parse().unwrap();
    let number_of_events: usize = first_line_split.next().unwrap().parse().unwrap();

    let mut events = Vec::new();

    for _ in 0..number_of_events {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();

        let event_type = parts.next().unwrap().to_string();
        let time = parts.next().unwrap().to_string();

        let mut third_part = String::new();
        if event_type == "MESSAGE" {
            let message_type = parts.next().unwrap().to_string();
            if message_type == "ALL" || message_type == "HERE" {
                third_part = message_type;
            } else {
                third_part = message_type;
                if let Some(rest_of_line) = parts.collect::<Vec<&str>>().join(" ").strip_prefix(' ') {
                    third_part.push_str(rest_of_line);
                }
            }
        } else if event_type == "OFFLINE" {
            third_part = parts.next().unwrap().to_string();
        }

        events.push(vec![event_type, time, third_part]);
    }

    let solution = Solution;
    let result = solution.count_mentions(number_of_users, events);

    for count in result {
        print!("{} ", count);
    }
    println!();
}