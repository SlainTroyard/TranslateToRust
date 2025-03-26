use std::io::{self, BufRead};

fn extract_digits(s: &str) -> Option<usize> {
    let mut num = 0;
    for c in s.chars() {
        if let Some(d) = c.to_digit(10) {
            num = num * 10 + d as usize;
        }
    }
    if num == 0 && s.is_empty() {
        None
    } else {
        Some(num)
    }
}

fn extract_id_number(s: &str) -> Option<usize> {
    let start = s.find(|c: char| c.is_digit(10))?;
    let digits_str = &s[start..];
    extract_digits(digits_str)
}

pub fn count_mentions(number_of_users: usize, events: Vec<Vec<String>>) -> Vec<usize> {
    let times: Vec<usize> = events.iter().map(|e| extract_digits(&e[1]).unwrap_or(0)).collect();

    let mut order: Vec<usize> = (0..events.len()).collect();
    order.sort_by_key(|&i| (times[i], if events[i][0].starts_with('O') { 0 } else { 1 }));

    let mut online = vec![0; number_of_users];
    let mut mention = vec![0; number_of_users];

    for &i in &order {
        let event = &events[i];
        let event_type = &event[0];
        if event_type.starts_with('M') {
            let third = &event[2];
            if third.starts_with('A') {
                for m in &mut mention {
                    *m += 1;
                }
            } else if third.starts_with('H') {
                let current_time = times[i];
                for j in 0..number_of_users {
                    let online_j = online[j];
                    if online_j == 0 {
                        mention[j] += 1;
                    } else if online_j <= current_time {
                        mention[j] += 1;
                        online[j] = 0;
                    }
                }
            } else {
                for token in third.split_whitespace() {
                    if let Some(user_id) = extract_id_number(token) {
                        if user_id < number_of_users {
                            mention[user_id] += 1;
                        }
                    }
                }
            }
        } else if event_type.starts_with('O') {
            let current_time = times[i];
            if let Some(user_id) = extract_id_number(&event[2]) {
                if user_id < number_of_users {
                    online[user_id] = current_time + 60;
                }
            }
        }
    }
    mention
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let number_of_users: usize = parts.next().unwrap().parse().unwrap();
    let events_size: usize = parts.next().unwrap().parse().unwrap();

    let mut events = Vec::with_capacity(events_size);
    for _ in 0..events_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let event: Vec<String> = parts.map(String::from).collect();
        events.push(event);
    }

    let mention = count_mentions(number_of_users, events);

    print!("Mentions: ");
    for &m in &mention {
        print!("{} ", m);
    }
    println!();
}