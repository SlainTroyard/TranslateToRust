use std::io::{self, BufRead};
use std::str::FromStr;

struct Solution;

impl Solution {
    // Compare function for sorting events
    fn cmp(a: &Vec<String>, b: &Vec<String>) -> std::cmp::Ordering {
        let time_a = a[1].parse::<i32>().unwrap();
        let time_b = b[1].parse::<i32>().unwrap();
        
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

    // Extract ID number from ID string
    fn extract_id(id_str: &str) -> i32 {
        // Check if string starts with "id"
        if id_str.starts_with("id") {
            match id_str[2..].parse::<i32>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Error parsing ID: {}, error: {}", id_str, e);
                    0 // Return default value
                }
            }
        } else {
            // Try to convert the entire string
            match id_str.parse::<i32>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Error parsing ID: {}, error: {}", id_str, e);
                    0 // Return default value
                }
            }
        }
    }

    // Update mention counts for specific users
    fn update_counts(counts: &mut Vec<i32>, mentions: &str) {
        for id_str in mentions.split_whitespace() {
            let id = Self::extract_id(id_str);
            if id >= 0 && (id as usize) < counts.len() {
                counts[id as usize] += 1;
            }
        }
    }

    // Main function to count mentions
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        let mut events = events;
        events.sort_by(Self::cmp);
        
        let number_of_users = number_of_users as usize;
        let mut online_times = vec![0; number_of_users];
        let mut counts = vec![0; number_of_users];
        let mut all_count = 0;
        
        for event in events {
            let time = event[1].parse::<i32>().unwrap();
            
            if event[0] == "OFFLINE" {
                match Self::extract_id(&event[2]) {
                    id if id >= 0 && (id as usize) < number_of_users => {
                        online_times[id as usize] = time + 60;
                    },
                    _ => {
                        eprintln!("Error processing OFFLINE event: ID out of range");
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
            for count in counts.iter_mut() {
                *count += all_count;
            }
        }
        
        counts
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read first line with number of users and events
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let number_of_users = parts.next().unwrap().parse::<i32>().unwrap();
    let number_of_events = parts.next().unwrap().parse::<i32>().unwrap();
    
    let mut events: Vec<Vec<String>> = Vec::with_capacity(number_of_events as usize);
    
    // Read events
    for _ in 0..number_of_events {
        let mut event = vec![String::new(), String::new(), String::new()];
        
        // Read event type and time
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        event[0] = parts.next().unwrap().to_string();
        event[1] = parts.next().unwrap().to_string();
        
        // Handle the third parameter based on event type
        if event[0] == "MESSAGE" {
            let message_type = parts.next().unwrap().to_string();
            
            if message_type == "ALL" || message_type == "HERE" {
                event[2] = message_type;
            } else {
                // This is a mention of users, collect the rest of the line
                let mut mentions = message_type;
                for part in parts {
                    mentions.push(' ');
                    mentions.push_str(part);
                }
                event[2] = mentions;
            }
        } else if event[0] == "OFFLINE" {
            event[2] = parts.next().unwrap().to_string();
        }
        
        events.push(event);
    }
    
    // Process events and print results
    match std::panic::catch_unwind(|| {
        let solution = Solution;
        let result = solution.count_mentions(number_of_users, events);
        
        let output: String = result.iter()
            .map(|count| count.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        
        println!("{}", output);
    }) {
        Ok(_) => {},
        Err(e) => {
            if let Some(s) = e.downcast_ref::<&str>() {
                eprintln!("Exception in main: {}", s);
            } else {
                eprintln!("Unknown exception in main");
            }
            std::process::exit(1);
        }
    }
}