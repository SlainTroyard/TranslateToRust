use std::io::{self, BufRead};
use std::str::FromStr;

struct Solution;

impl Solution {
    fn cmp(a: &[String], b: &[String]) -> bool {
        let time_a = a[1].parse::<i32>().unwrap();
        let time_b = b[1].parse::<i32>().unwrap();
        (time_a == time_b && a[0] != b[0]) && a[0] == "OFFLINE" || time_a < time_b
    }

    // Extract the numeric part of the ID
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

    fn update_counts(counts: &mut [i32], mentions: &str) {
        for id_str in mentions.split_whitespace() {
            if let Ok(id) = id_str.parse::<i32>() {
                if id >= 0 && id < counts.len() as i32 {
                    counts[id as usize] += 1;
                }
            } else {
                eprintln!("Error in update_counts: {}", id_str);
            }
        }
    }

    pub fn count_mentions(number_of_users: i32, events: &mut [Vec<String>]) -> Vec<i32> {
        events.sort_by(|a, b| Solution::cmp(a, b).then(a[0].cmp(&b[0])));
        let mut online_times = vec![0; number_of_users as usize];
        let mut counts = vec![0; number_of_users as usize];
        let mut all_count = 0;

        for event in events {
            let time = event[1].parse::<i32>().unwrap();
            if event[0] == "OFFLINE" {
                let id = Solution::extract_id(&event[2]);
                if id >= 0 && id < number_of_users {
                    online_times[id as usize] = time + 60;
                }
                continue;
            }

            let mentions = &event[2];
            if mentions == "ALL" {
                all_count += 1;
            } else if mentions == "HERE" {
                for i in 0..number_of_users as usize {
                    if online_times[i] <= time as usize {
                        counts[i] += 1;
                    }
                }
            } else {
                Solution::update_counts(&mut counts, mentions);
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap()?;
    let mut numbers = first_line.split_whitespace();
    let number_of_users = numbers.next().unwrap().parse::<i32>().unwrap();
    let number_of_events = numbers.next().unwrap().parse::<i32>().unwrap();

    let mut events = Vec::with_capacity(number_of_events as usize);
    for _ in 0..number_of_events {
        let mut event = Vec::with_capacity(3);
        let event_type = lines.next().unwrap()?;
        let mut event_type_words = event_type.split_whitespace();
        event.push(event_type_words.next().unwrap().to_string());
        event.push(event_type_words.next().unwrap().to_string());

        if event[0] == "MESSAGE" {
            let message_type = event_type_words.next().unwrap().to_string();
            if message_type == "ALL" || message_type == "HERE" {
                event.push(message_type);
            } else {
                event.push(message_type);
                if let Some(rest_of_line) = event_type_words.next() {
                    event[2].push_str(" ");
                    event[2].push_str(rest_of_line);
                }
                if let Some(rest_of_line) = lines.next() {
                    let rest_of_line = rest_of_line?;
                    if !rest_of_line.is_empty() {
                        event[2].push_str(" ");
                        event[2].push_str(&rest_of_line);
                    }
                }
            }
        } else if event[0] == "OFFLINE" {
            event.push(event_type_words.next().unwrap().to_string());
        }
        events.push(event);
    }

    let solution = Solution;
    let result = solution.count_mentions(number_of_users, &mut events);

    for count in result {
        print!("{} ", count);
    }
    println!();

    Ok(())
}