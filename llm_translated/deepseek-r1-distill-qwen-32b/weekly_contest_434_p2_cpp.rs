use std::io;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn count_mentions(number_of_users: usize, events: Vec<Vec<String>>) -> Vec<i32> {
        let mut events = events.clone();
        events.sort_by(|a, b| {
            let time_a = a[1].parse::<i32>().unwrap_or(0);
            let time_b = b[1].parse::<i32>().unwrap_or(0);
            if time_a == time_b {
                if a[0] == "OFFLINE" && b[0] != "OFFLINE" {
                    Ordering::Less
                } else if a[0] != "OFFLINE" && b[0] == "OFFLINE" {
                    Ordering::Greater
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
        
        for event in events {
            let time = event[1].parse::<i32>().unwrap_or(0);
            if event[0] == "OFFLINE" {
                if let Ok(id) = extract_id(&event[2]) {
                    if id >= 0 && id < number_of_users as i32 {
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
}

fn extract_id(id_str: &str) -> Result<i32, std::num::ParseIntError> {
    if id_str.starts_with("id") {
        id_str[2..].parse::<i32>()
    } else {
        id_str.parse::<i32>()
    }
}

fn update_counts(counts: &mut Vec<i32>, mentions: &str) {
    for id_str in mentions.split_whitespace() {
        if let Ok(id) = extract_id(id_str) {
            if id >= 0 && id < counts.len() as i32 {
                counts[id as usize] += 1;
            }
        }
    }
}

fn main() {
    let mut input = io::stdin().lock();
    let mut buffer = VecDeque::new();
    
    let mut first_line = String::new();
    input.read_line(&mut first_line).unwrap();
    let mut parts = first_line.split_whitespace();
    let number_of_users = parts.next().and_then(|s| usize::from_str(s).ok()).unwrap();
    let number_of_events = parts.next().and_then(|s| usize::from_str(s).ok()).unwrap();
    
    for _ in 0..number_of_events {
        let mut line = String::new();
        input.read_line(&mut line).unwrap();
        let mut parts = line.split_whitespace().map(|s| s.to_string());
        let event_type = parts.next().unwrap();
        let time = parts.next().unwrap();
        
        if event_type == "MESSAGE" {
            let mut message_type = parts.next().unwrap();
            let mut mentions = message_type.clone();
            if message_type != "ALL" && message_type != "HERE" {
                let mut rest = parts.collect::<Vec<String>>().join(" ");
                if !rest.is_empty() {
                    mentions += &rest;
                }
            }
            buffer.push_back(vec![event_type, time, mentions]);
        } else if event_type == "OFFLINE" {
            let id = parts.next().unwrap();
            buffer.push_back(vec![event_type, time, id]);
        }
    }
    
    let events = buffer.into_iter().map(|v| v).collect::<Vec<Vec<String>>>();
    let solution = Solution;
    let result = solution.count_mentions(number_of_users, events);
    
    println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}