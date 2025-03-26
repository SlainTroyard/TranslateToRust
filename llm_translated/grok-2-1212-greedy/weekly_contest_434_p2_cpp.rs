use std::io::{self, BufRead};
use std::str::FromStr;

struct Solution;

impl Solution {
    fn cmp(a: &[String], b: &[String]) -> bool {
        let time_a = a[1].parse::<i32>().unwrap();
        let time_b = b[1].parse::<i32>().unwrap();
        (time_a == time_b && a[0] != b[0]) && a[0] == "OFFLINE" || time_a < time_b
    }

    fn extract_id(id_str: &str) -> i32 {
        if id_str.starts_with("id") {
            id_str[2..].parse().unwrap_or_else(|_| {
                eprintln!("Error parsing ID: {}", id_str);
                0
            })
        } else {
            id_str.parse().unwrap_or_else(|_| {
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
                let id = Solution::extract_id(id_str);
                if id >= 0 && id < counts.len() as i32 {
                    counts[id as usize] += 1;
                }
            }
        }
    }

    fn count_mentions(number_of_users: i32, events: &mut [Vec<String>]) -> Vec<i32> {
        events.sort_by(|a, b| Solution::cmp(a, b).then_with(|| a.cmp(b)));
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
    let number_of_events = numbers.next().unwrap().parse::<usize>().unwrap();

    let mut events = Vec::with_capacity(number_of_events);
    for _ in 0..number_of_events {
        let event_type = lines.next().unwrap()?;
        let time = lines.next().unwrap()?;
        let mut event = vec![event_type, time];

        if event[0] == "MESSAGE" {
            let message_type = lines.next().unwrap()?;
            if message_type == "ALL" || message_type == "HERE" {
                event.push(message_type);
            } else {
                event.push(message_type);
                let rest_of_line = lines.next().unwrap()?;
                if !rest_of_line.is_empty() {
                    event[2].push_str(&rest_of_line);
                }
            }
        } else if event[0] == "OFFLINE" {
            let id = lines.next().unwrap()?;
            event.push(id);
        }

        events.push(event);
    }

    let result = Solution::count_mentions(number_of_users, &mut events);
    for count in result {
        print!("{} ", count);
    }
    println!();

    Ok(())
}