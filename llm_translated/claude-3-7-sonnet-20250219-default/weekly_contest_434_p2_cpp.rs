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

    // Extract ID from string
    fn extract_id(id_str: &str) -> i32 {
        // Check if string starts with "id"
        if id_str.starts_with("id") {
            id_str[2..].parse::<i32>().unwrap_or_else(|e| {
                eprintln!("Error parsing ID: {}, {}", id_str, e);
                0
            })
        } else {
            // Try to convert the entire string
            id_str.parse::<i32>().unwrap_or_else(|e| {
                eprintln!("Error parsing ID: {}, {}", id_str, e);
                0
            })
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
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        let mut events = events;
        events.sort_by(|a, b| Self::cmp(a, b));
        
        let number_of_users = number_of_users as usize;
        let mut online_times = vec![0; number_of_users];
        let mut counts = vec![0; number_of_users];
        let mut all_count = 0;
        
        for event in events {
            let time = event[1].parse::<i32>().unwrap();
            
            if event[0] == "OFFLINE" {
                let id = Self::extract_id(&event[2]);
                if id >= 0 && (id as usize) < number_of_users {
                    online_times[id as usize] = time + 60;
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line
    let first_line = lines.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    let number_of_users = parts.next().unwrap().parse::<i32>().unwrap();
    let number_of_events = parts.next().unwrap().parse::<i32>().unwrap();
    
    let mut events = Vec::new();
    
    // Read events
    for _ in 0..number_of_events {
        let line = lines.next().unwrap()?;
        let mut parts = line.split_whitespace();
        let event_type = parts.next().unwrap().to_string();
        let time = parts.next().unwrap().to_string();
        
        if event_type == "MESSAGE" {
            let message_type = parts.next().unwrap().to_string();
            
            if message_type == "ALL" || message_type == "HERE" {
                events.push(vec![event_type, time, message_type]);
            } else {
                // This is a case with mentions, collect the rest of the line
                let mut mentions = message_type;
                for part in parts {
                    mentions.push(' ');
                    mentions.push_str(part);
                }
                events.push(vec![event_type.clone(), time, mentions]);
            }
        } else if event_type == "OFFLINE" {
            let user_id = parts.next().unwrap().to_string();
            events.push(vec![event_type, time, user_id]);
        }
    }
    
    // Process and output result
    let result = Solution::count_mentions(number_of_users, events);
    
    for count in result {
        print!("{} ", count);
    }
    println!();
    
    Ok(())
}