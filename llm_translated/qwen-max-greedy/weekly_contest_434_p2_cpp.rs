use std::io::{self, BufRead, Write};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the number of users and events
    stdin.read_line(&mut buffer).unwrap();
    let mut iter = buffer.split_whitespace();
    let number_of_users: usize = iter.next().unwrap().parse().unwrap();
    let number_of_events: usize = iter.next().unwrap().parse().unwrap();
    buffer.clear();

    // Read the events
    let mut events = vec![vec!["".to_string(); 3]; number_of_events];
    for i in 0..number_of_events {
        stdin.read_line(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace();
        events[i][0] = iter.next().unwrap().to_string();
        events[i][1] = iter.next().unwrap().to_string();

        if events[i][0] == "MESSAGE" {
            let message_type = iter.next().unwrap().to_string();
            if message_type == "ALL" || message_type == "HERE" {
                events[i][2] = message_type;
            } else {
                events[i][2] = message_type;
                let rest_of_line = stdin.lock().lines().next().unwrap().unwrap();
                if !rest_of_line.trim().is_empty() {
                    events[i][2].push(' ');
                    events[i][2].push_str(&rest_of_line);
                }
            }
        } else if events[i][0] == "OFFLINE" {
            events[i][2] = iter.next().unwrap().to_string();
        }

        buffer.clear();
    }

    // Process the events
    let solution = Solution {};
    let result = solution.count_mentions(number_of_users, events);

    // Output the results
    for count in result {
        write!(stdout, "{} ", count).unwrap();
    }
    writeln!(stdout).unwrap();
}

struct Solution;

impl Solution {
    fn cmp(a: &Vec<String>, b: &Vec<String>) -> bool {
        let time_a = a[1].parse::<i32>().unwrap();
        let time_b = b[1].parse::<i32>().unwrap();
        (time_a == time_b && a[0] != b[0]) || (time_a < time_b)
    }

    fn extract_id(id_str: &str) -> i32 {
        if id_str.starts_with("id") {
            match id_str[2..].parse::<i32>() {
                Ok(id) => id,
                Err(_) => {
                    eprintln!("Error parsing ID: {}", id_str);
                    0
                }
            }
        } else {
            match id_str.parse::<i32>() {
                Ok(id) => id,
                Err(_) => {
                    eprintln!("Error parsing ID: {}", id_str);
                    0
                }
            }
        }
    }

    fn update_counts(counts: &mut Vec<i32>, mentions: &str) {
        for id_str in mentions.split_whitespace() {
            let id = Self::extract_id(id_str);
            if id >= 0 && (id as usize) < counts.len() {
                counts[id as usize] += 1;
            }
        }
    }

    pub fn count_mentions(&self, number_of_users: usize, mut events: Vec<Vec<String>>) -> Vec<i32> {
        events.sort_by(|a, b| Self::cmp(a, b).then_with(|| a.cmp(b)));
        let mut online_times = vec![0; number_of_users];
        let mut counts = vec![0; number_of_users];
        let mut all_count = 0;

        for event in &events {
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
                for (i, &online_time) in online_times.iter().enumerate() {
                    if online_time <= time {
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