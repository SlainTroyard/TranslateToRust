use std::cmp::Ordering;
use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    fn cmp(a: &Vec<String>, b: &Vec<String>) -> Ordering {
        let time_a = a[1].parse::<i32>().unwrap();
        let time_b = b[1].parse::<i32>().unwrap();

        if time_a == time_b {
            if a[0] != b[0] {
                if a[0] == "OFFLINE" {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Equal
            }
        } else {
            time_a.cmp(&time_b)
        }
    }

    fn extract_id(&self, id_str: &str) -> i32 {
        if id_str.starts_with("id") {
            match id_str[2..].parse::<i32>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Error parsing ID: {}. Error: {}", id_str, e);
                    0 // Return default value
                }
            }
        } else {
            match id_str.parse::<i32>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Error parsing ID: {}. Error: {}", id_str, e);
                    0 // Return default value
                }
            }
        }
    }

    fn update_counts(&self, counts: &mut Vec<i32>, mentions: &str) {
        for id_str in mentions.split_whitespace() {
            match self.extract_id(id_str) {
                id if id >= 0 && id < counts.len() as i32 => {
                    counts[id as usize] += 1;
                }
                _ => {
                    eprintln!("Invalid id: {}", id_str);
                }
            }
        }
    }

    pub fn count_mentions(&self, number_of_users: i32, events: &mut Vec<Vec<String>>) -> Vec<i32> {
        events.sort_by(|a, b| Solution::cmp(a, b));
        let mut online_times = vec![0; number_of_users as usize];
        let mut counts = vec![0; number_of_users as usize];
        let mut all_count = 0;

        for event in events {
            let time = event[1].parse::<i32>().unwrap();

            if event[0] == "OFFLINE" {
                match self.extract_id(&event[2]) {
                    id if id >= 0 && id < number_of_users => {
                        online_times[id as usize] = time + 60;
                    }
                    _ => {
                        eprintln!("Error processing OFFLINE event: invalid id");
                    }
                }
                continue;
            }

            let mentions = &event[2];
            if mentions == "ALL" {
                all_count += 1;
            } else if mentions == "HERE" {
                for i in 0..number_of_users {
                    if online_times[i as usize] <= time {
                        counts[i as usize] += 1;
                    }
                }
            } else {
                self.update_counts(&mut counts, mentions);
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
    let mut iterator = stdin.lock().lines();

    let first_line = iterator.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();

    let number_of_users = first_line_iter.next().unwrap().parse::<i32>().unwrap();
    let number_of_events = first_line_iter.next().unwrap().parse::<i32>().unwrap();

    let mut events: Vec<Vec<String>> = Vec::new();

    for _ in 0..number_of_events {
        let mut line = iterator.next().unwrap().unwrap();
        let mut event: Vec<String> = Vec::new();

        let mut parts = line.split_whitespace().collect::<Vec<&str>>();

        event.push(parts[0].to_string());
        event.push(parts[1].to_string());

        if parts[0] == "MESSAGE" {
            if parts.len() > 2 {
                let message_type = parts[2].to_string();

                if message_type == "ALL" || message_type == "HERE" {
                    event.push(message_type);
                } else {
                    let mut mentions = parts[2].to_string();
                    
                    if parts.len() > 3 {
                        let mut rest_of_line = String::new();
                        for i in 3..parts.len(){
                            rest_of_line.push_str(parts[i]);
                            if i < parts.len() - 1 {
                                rest_of_line.push(' ');
                            }
                        }
                        mentions.push(' ');
                        mentions.push_str(&rest_of_line);
                        event.push(mentions);
                    } else {
                        event.push(message_type);
                    }
                }
            }
        } else if parts[0] == "OFFLINE" {
            event.push(parts[2].to_string());
        }

        events.push(event);
    }

    let solution = Solution {};
    let result = solution.count_mentions(number_of_users, &mut events);

    for count in result {
        print!("{} ", count);
    }
    println!();
}