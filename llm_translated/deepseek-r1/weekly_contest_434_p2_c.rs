use std::io::{self, Read};

fn str_to_num(s: &str) -> i32 {
    let mut num = 0;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = num * 10 + (c as i32 - '0' as i32);
        }
    }
    num
}

fn extract_id_number(s: &str) -> i32 {
    match s.find(|c: char| c.is_ascii_digit()) {
        Some(start) => str_to_num(&s[start..]),
        None => -1,
    }
}

fn count_mentions(number_of_users: usize, events: &[Vec<String>]) -> Vec<i32> {
    let events_size = events.len();
    let time_arr: Vec<i32> = events.iter().map(|e| str_to_num(&e[1])).collect();
    
    // Create and sort event indices based on time and event type
    let mut order: Vec<usize> = (0..events_size).collect();
    order.sort_by(|&a, &b| {
        let time_cmp = time_arr[a].cmp(&time_arr[b]);
        let event_cmp = events[b][0].as_bytes()[0].cmp(&events[a][0].as_bytes()[0]);
        time_cmp.then(event_cmp)
    });

    let mut online = vec![0; number_of_users];
    let mut mention = vec![0; number_of_users];

    for &i in &order {
        let event = &events[i];
        let event_type = &event[0];
        let current_time = time_arr[i];

        if event_type.starts_with('M') {
            let target = &event[2];
            if target.starts_with('A') {
                // Mention ALL: increment all users
                mention.iter_mut().for_each(|m| *m += 1);
            } else if target.starts_with('H') {
                // Mention HERE
                for j in 0..number_of_users {
                    if online[j] == 0 || online[j] <= current_time {
                        if online[j] != 0 && online[j] <= current_time {
                            online[j] = 0;
                        }
                        mention[j] += 1;
                    }
                }
            } else {
                // Mention specific users
                for part in event[2].split(' ').filter(|s| !s.is_empty()) {
                    let user_id = extract_id_number(part);
                    if user_id >= 0 && (user_id as usize) < number_of_users {
                        mention[user_id as usize] += 1;
                    }
                }
            }
        } else if event_type.starts_with('O') {
            // Online event
            let user_id = extract_id_number(&event[2]);
            if user_id >= 0 && (user_id as usize) < number_of_users {
                online[user_id as usize] = current_time + 60;
            }
        }
    }

    mention
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    let number_of_users: usize = tokens.next().and_then(|s| s.parse().ok()).unwrap();
    let events_size: usize = tokens.next().and_then(|s| s.parse().ok()).unwrap();

    let mut events = Vec::with_capacity(events_size);
    for _ in 0..events_size {
        let event_type = tokens.next().unwrap().to_string();
        let time_str = tokens.next().unwrap().to_string();
        let target = tokens.next().unwrap().to_string();
        events.push(vec![event_type, time_str, target]);
    }

    let mentions = count_mentions(number_of_users, &events);

    print!("Mentions: ");
    for (i, &count) in mentions.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", count);
    }
    println!();
}