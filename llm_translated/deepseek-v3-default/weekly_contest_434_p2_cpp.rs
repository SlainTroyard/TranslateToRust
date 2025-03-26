use std::io::{self, BufRead};
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct Solution;

impl Solution {
    // Comparator function to sort events
    fn cmp(a: &Vec<String>, b: &Vec<String>) -> Ordering {
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
    }

    // Extract ID from the string
    fn extract_id(id_str: &str) -> i32 {
        if id_str.starts_with("id") {
            id_str[2..].parse().unwrap_or(0)
        } else {
            id_str.parse().unwrap_or(0)
        }
    }

    // Update counts based on mentions
    fn update_counts(counts: &mut Vec<i32>, mentions: &str) {
        for id_str in mentions.split_whitespace() {
            let id = Self::extract_id(id_str);
            if id >= 0 && (id as usize) < counts.len() {
                counts[id as usize] += 1;
            }
        }
    }

    // Main function to count mentions
    fn count_mentions(number_of_users: i32, events: &mut Vec<Vec<String>>) -> Vec<i32> {
        events.sort_by(Self::cmp);
        let mut online_times = vec![0; number_of_users as usize];
        let mut counts = vec![0; number_of_users as usize];
        let mut all_count = 0;

        for event in events {
            let time = event[1].parse::<i32>().unwrap();
            if event[0] == "OFFLINE" {
                let id = Self::extract_id(&event[2]);
                if id >= 0 && (id as usize) < online_times.len() {
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
    let mut parts = first_line.split_whitespace();
    let number_of_users = parts.next().unwrap().parse::<i32>().unwrap();
    let number_of_events = parts.next().unwrap().parse::<i32>().unwrap();
    
    let mut events = Vec::new();
    for _ in 0..number_of_events {
        let line = lines.next().unwrap().unwrap();
        let mut event = line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        
        if event[0] == "MESSAGE" {
            let message_type = lines.next().unwrap().unwrap();
            if message_type == "ALL" || message_type == "HERE" {
                event.push(message_type);
            } else {
                let rest_of_line = lines.next().unwrap().unwrap();
                event.push(format!("{} {}", message_type, rest_of_line));
            }
        } else if event[0] == "OFFLINE" {
            let id = lines.next().unwrap().unwrap();
            event.push(id);
        }
        
        events.push(event);
    }
    
    let result = Solution::count_mentions(number_of_users, &mut events);
    for count in result {
        print!("{} ", count);
    }
    println!();
}