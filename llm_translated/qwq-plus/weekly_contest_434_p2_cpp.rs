use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn count_mentions(&self, number_of_users: usize, events: &[Vec<String>]) -> Vec<usize> {
        let mut events_sorted = events.to_vec();
        events_sorted.sort_by(|a, b| {
            let time_a: i32 = a[1].parse().unwrap();
            let time_b: i32 = b[1].parse().unwrap();
            if time_a != time_b {
                time_a.cmp(&time_b)
            } else if a[0] == b[0] {
                std::cmp::Ordering::Equal
            } else {
                if a[0] == "OFFLINE" {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            }
        });

        let mut online_times = vec![0; number_of_users];
        let mut counts = vec![0; number_of_users];
        let mut all_count = 0;

        for event in &events_sorted {
            let time_str = &event[1];
            let time: i32 = time_str.parse().unwrap();

            if event[0] == "OFFLINE" {
                let id_str = &event[2];
                let id = Self::extract_id(id_str);
                if id >= 0 && (id as usize) < number_of_users {
                    online_times[id as usize] = time + 60;
                }
            } else {
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
        }

        if all_count > 0 {
            for count in &mut counts {
                *count += all_count;
            }
        }

        counts
    }

    fn extract_id(id_str: &str) -> i32 {
        if id_str.starts_with("id") {
            id_str[2..].parse().unwrap_or(0)
        } else {
            id_str.parse().unwrap_or(0)
        }
    }

    fn update_counts(counts: &mut Vec<usize>, mentions: &str) {
        for id_str in mentions.split_whitespace() {
            let id = Self::extract_id(id_str);
            if id >= 0 && (id as usize) < counts.len() {
                counts[id as usize] += 1;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let number_of_users = parts[0].parse().unwrap();
    let number_of_events = parts[1].parse().unwrap();

    let mut events = Vec::with_capacity(number_of_events);

    for _ in 0..number_of_events {
        let line = lines.next().unwrap().unwrap();
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let event_type = tokens[0].to_string();
        let time = tokens[1].to_string();
        let mentions = if tokens.len() >= 3 {
            tokens[2..].join(" ")
        } else {
            "".to_string()
        };
        events.push(vec![event_type, time, mentions]);
    }

    let solution = Solution;
    let result = solution.count_mentions(number_of_users, &events);
    for count in &result {
        print!("{} ", count);
    }
    println!();
}