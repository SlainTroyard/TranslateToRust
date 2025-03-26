use std::io::{self, BufRead};
use std::cmp::Ordering;
use std::error::Error;

struct Solution;

impl Solution {
    // Extract the ID number from a string like "id123" or just "123"
    fn extract_id(id_str: &str) -> Option<usize> {
        if id_str.starts_with("id") {
            id_str[2..].parse::<usize>().ok()
        } else {
            id_str.parse::<usize>().ok()
        }
    }

    fn update_counts(counts: &mut [i32], mentions: &str) {
        for id_str in mentions.split_whitespace() {
            if let Some(id) = Self::extract_id(id_str) {
                if id < counts.len() {
                    counts[id] += 1;
                }
            }
        }
    }

    pub fn count_mentions(number_of_users: usize, mut events: Vec<Vec<String>>) -> Vec<i32> {
        // Sort events by time, with OFFLINE events coming first in case of ties
        events.sort_by(|a, b| {
            let time_a = a[1].parse::<i32>().unwrap_or(0);
            let time_b = b[1].parse::<i32>().unwrap_or(0);
            
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

        let mut online_times = vec![0; number_of_users];
        let mut counts = vec![0; number_of_users];
        let mut all_count = 0;
        
        for event in events {
            let time = event[1].parse::<i32>().unwrap_or(0);
            
            if event[0] == "OFFLINE" {
                if let Some(id) = Self::extract_id(&event[2]) {
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

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut input = String::new();
    
    // Read the first line with number of users and events
    stdin.lock().read_line(&mut input)?;
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let number_of_users: usize = parts[0].parse()?;
    let number_of_events: usize = parts[1].parse()?;
    
    let mut events = Vec::with_capacity(number_of_events);
    let mut line = String::new();
    
    for _ in 0..number_of_events {
        line.clear();
        stdin.lock().read_line(&mut line)?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        
        let event_type = parts[0].to_string();
        let time = parts[1].to_string();
        
        let mut event = vec![event_type.clone(), time];
        
        if event_type == "MESSAGE" {
            if parts.len() > 2 {
                let message_type = parts[2];
                
                if message_type == "ALL" || message_type == "HERE" {
                    event.push(message_type.to_string());
                } else {
                    // This is a mention case, we need the rest of line including the first mention
                    let mentions_start = line.find(message_type).unwrap_or(0);
                    let mentions = line[mentions_start..].trim().to_string();
                    event.push(mentions);
                }
            }
        } else if event_type == "OFFLINE" {
            if parts.len() > 2 {
                let user_id = parts[2].to_string();
                event.push(user_id);
            }
        }
        
        events.push(event);
    }
    
    let result = Solution::count_mentions(number_of_users, events);
    
    // Print result with spaces between numbers
    for (i, count) in result.iter().enumerate() {
        print!("{}", count);
        if i < result.len() - 1 {
            print!(" ");
        }
    }
    println!();
    
    Ok(())
}